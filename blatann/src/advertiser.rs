use std::sync::{Arc, Mutex};

use blatann_event::{Subscribable, Subscriber, SubscriberAction, Publisher};
use blatann_event::event_waitable::EventWaitable;

use nrf_driver::driver::NrfDriver;
use nrf_driver::error::{NrfError, NrfErrorType, NrfResult};
use nrf_driver::gap::enums::*;
use nrf_driver::gap::events::GapEventTimeout;
use nrf_driver::gap::types::*;
use nrf_driver::utils::Milliseconds;

use crate::events::{AdvertisingTimeoutEvent, ConnectionEvent, DisconnectionEvent};
use crate::advertise_data::{AdvData, MAX_ADVERTISE_ENCODED_LEN};
use crate::peer::Peer;

pub type AdvType = BleGapAdvertisingType;

pub type AdvWaitableResult<E> = Result<Arc<EventWaitable<Advertiser, E>>, NrfError>;

pub const ADVERTISE_FOREVER: u16 = 0;

struct AdvState {
    is_advertising: bool,
    auto_restart: bool,
    adv_type: AdvType,
    interval: Milliseconds,
    timeout_s: u16,
}

impl Default for AdvState {
    fn default() -> Self {
        Self {
            is_advertising: false,
            auto_restart: false,
            adv_type: AdvType::ConnectableUndirected,
            interval: 100_f64,
            timeout_s: ADVERTISE_FOREVER
        }
    }
}

pub struct Advertiser {
    driver: Arc<NrfDriver>,
    central: Arc<Peer>,
    pub on_timeout: Publisher<Self, AdvertisingTimeoutEvent>,
    state: Mutex<AdvState>,
}


impl Advertiser {
    pub(crate) fn new(driver: &Arc<NrfDriver>, central: &Arc<Peer>) -> Arc<Self> {
        let advertiser = Arc::new(Self {
            driver: driver.clone(),
            central: central.clone(),
            on_timeout: Publisher::new("Advertising Timeout"),
            state: Mutex::new(Default::default()),
        });

        driver.events.gap_timeout.subscribe(advertiser.clone());
        central.on_connect.subscribe(advertiser.clone());
        central.on_disconnect.subscribe(advertiser.clone());

        return advertiser;
    }

    pub fn set_params(&self, interval: Milliseconds, timeout_s: u16, adv_type: AdvType, auto_restart: bool) {
        let mut state = self.state.lock().unwrap();
        state.interval = interval;
        state.timeout_s = timeout_s;
        state.adv_type = adv_type;
        state.auto_restart = auto_restart;
    }

    pub fn set_data(&self, advertise_data: Option<&AdvData>, scan_response: Option<&AdvData>) -> Result<(), NrfError> {
        let adv_data = advertise_data.and_then(|d| { Some(d.serialize()) });
        let scan_data = scan_response.and_then(|d| { Some(d.serialize()) });

        if let Some(a) = &adv_data {
            if a.len() > MAX_ADVERTISE_ENCODED_LEN {
                return NrfErrorType::DataSize.to_result();
            }
        };
        if let Some(a) = &scan_data {
            if a.len() > MAX_ADVERTISE_ENCODED_LEN {
                return NrfErrorType::DataSize.to_result();
            }
        };

        self.driver.ble_gap_adv_data_set(&adv_data, &scan_data)
    }

    pub fn start(&self) -> AdvWaitableResult<AdvertisingTimeoutEvent> {
        self._stop().and_then(|_| {
            self._start()
        }).and_then(|_| {
            Ok(EventWaitable::new(&self.on_timeout))
        })
    }

    fn _start(&self) -> NrfResult<()> {
        let mut state = self.state.lock().unwrap();
        let params = BleGapAdvParams::new(state.interval, state.timeout_s, state.adv_type);

        self.driver.ble_gap_adv_start(&params).and_then(|_| {
            state.is_advertising = true;
            Ok(())
        })
    }

    pub fn stop(&self) -> Result<(), NrfError> {
        let mut state = self.state.lock().unwrap();
        state.auto_restart = false;
        drop(state);

        self._stop()
    }

    fn _stop(&self) -> NrfResult<()> {
        let mut state = self.state.lock().unwrap();
        state.is_advertising = false;
        match self.driver.ble_gap_adv_stop() {
            Ok(_) => Ok(()),
            Err(e) => match e.error_type {
                NrfErrorType::Success => Ok(()),
                NrfErrorType::InvalidState => Ok(()),
                _ => Err(e)
            }
        }
    }
}

impl Subscriber<Peer, ConnectionEvent> for Advertiser {
    fn handle(self: Arc<Self>, sender: Arc<Peer>, event: ConnectionEvent) -> Option<SubscriberAction> {
        let mut state = self.state.lock().unwrap();
        state.is_advertising = false;
        return None;
    }
}

impl Subscriber<Peer, DisconnectionEvent> for Advertiser {
    fn handle(self: Arc<Self>, _sender: Arc<Peer>, _event: DisconnectionEvent) -> Option<SubscriberAction> {
        let auto_restart_enabled = {
            self.state.lock().unwrap().auto_restart
        };

        if auto_restart_enabled {
            info!("Re-enabling advertising after disconnect");
            self._start().unwrap_or_else(|e| {
                warn!("Failed to auto-restart with error {:?}", e);
            });
        }

        return None;
    }
}

impl Subscriber<NrfDriver, GapEventTimeout> for Advertiser {
    fn handle(self: Arc<Self>, _sender: Arc<NrfDriver>, event: GapEventTimeout) -> Option<SubscriberAction> {
        if let BleGapTimeoutSource::Advertising = event.src {
            // Notify that advertising timed out first which may call stop() to disable auto-restart
            self.on_timeout.dispatch(self.clone(), AdvertisingTimeoutEvent {});

            let mut state = self.state.lock().unwrap();
            if state.auto_restart {
                let params = BleGapAdvParams::new(state.interval, state.timeout_s, state.adv_type);
                self.driver.ble_gap_adv_start(&params).unwrap_or_else(|e| {
                    warn!("Failed to auto-restart with error {:?}", e);
                });
            } else {
                state.is_advertising = false;
            }
        }
        return None;
    }
}
