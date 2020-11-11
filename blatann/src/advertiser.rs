use std::sync::Arc;

use blatann_event::{Subscribable, Subscriber, SubscriberAction, Publisher};
use blatann_event::event_waitable::EventWaitable;

use nrf_driver::driver::NrfDriver;
use nrf_driver::error::{NrfError, NrfErrorType};
use nrf_driver::gap::enums::*;
use nrf_driver::gap::events::BleGapTimeout;
use nrf_driver::gap::types::*;
use nrf_driver::utils::Milliseconds;

use crate::events::AdvertisingTimeoutEvent;

pub type AdvertisingType = BleGapAdvertisingType;

pub type AdvWaitableResult<E> = Result<Arc<EventWaitable<Advertiser, E>>, NrfError>;

pub struct Advertiser {
    driver: Arc<NrfDriver>,
    pub on_timeout: Publisher<Self, AdvertisingTimeoutEvent>,
}


impl Advertiser {
    pub fn new(driver: Arc<NrfDriver>) -> Arc<Self> {
        let advertiser = Arc::new(Self {
            driver: driver.clone(),
            on_timeout: Publisher::new("Advertising Timeout"),
        });

        driver.events.gap_timeout.subscribe(advertiser.clone());

        return advertiser;
    }

    pub fn start(&self, interval: Milliseconds, timeout_s: u16, adv_type: AdvertisingType) -> AdvWaitableResult<AdvertisingTimeoutEvent> {
        let params = BleGapAdvParams::new(interval, timeout_s, adv_type);

        self.driver.ble_gap_adv_start(&params).and_then(|_| {
            Ok(EventWaitable::new(&self.on_timeout))
        })
    }

    pub fn stop(&self) -> Result<(), NrfError> {
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

impl Subscriber<NrfDriver, BleGapTimeout> for Advertiser {
    fn handle(self: Arc<Self>, _sender: Arc<NrfDriver>, event: BleGapTimeout) -> Option<SubscriberAction> {
        if let BleGapTimeoutSource::Advertising = event.src {
            self.on_timeout.dispatch(self.clone(), AdvertisingTimeoutEvent {});
        }
        return None;
    }
}
