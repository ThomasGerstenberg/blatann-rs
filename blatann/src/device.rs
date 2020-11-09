use nrf_driver::driver::NrfDriver;
use nrf_driver::DRIVER_MANAGER;
use std::sync::Arc;
use nrf_driver::event_publisher::{EventHandler, Subscribable, EventPublisher};
use nrf_driver::gap::events::BleGapTimeout;
use nrf_driver::utils::Milliseconds;
use nrf_driver::gap::enums::*;
use nrf_driver::gap::types::*;
use nrf_driver::error::{NrfError, NrfErrorType};
use crate::event_waitable::{EventWaitableResult, EventWaitable};

pub type AdvertisingType = BleGapAdvertisingType;

pub struct Advertiser {
    driver: Arc<NrfDriver>,
    on_timeout: EventPublisher<Self, ()>
}


impl Advertiser {
    pub fn new(driver: Arc<NrfDriver>) -> Arc<Self> {
        let advertiser = Arc::new(Self {
            driver: driver.clone(),
            on_timeout: EventPublisher::new(),
        });

        driver.events.gap_timeout.subscribe(advertiser.clone());

        return advertiser;
    }

    pub fn start(&self, interval: Milliseconds, timeout_s: u16, adv_type: AdvertisingType) -> EventWaitableResult<Self, ()> {
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

impl EventHandler<NrfDriver, BleGapTimeout> for Advertiser {
    fn handle(self: Arc<Self>, _sender: Arc<NrfDriver>, event: BleGapTimeout) {
        if let BleGapTimeoutSource::Advertising = event.src {
            println!("Got advertising timeout event");
            self.on_timeout.dispatch(self.clone(), ());
        }
    }
}

pub struct BleDevice {
    pub port: String,
    pub driver: Arc<NrfDriver>,
    pub advertiser: Arc<Advertiser>
}


impl BleDevice {
    pub fn new(port: String, baud: u32) -> Self {
        let driver = {
            let mut manager = DRIVER_MANAGER.lock().unwrap();
            manager.create(port.clone(), baud, false)
        };
        let advertiser = Advertiser::new(driver.clone());

        Self {
            port,
            driver,
            advertiser
        }
    }

    pub fn open(&self) -> Result<(), NrfError> {
        self.driver.open().and_then(|_| self.driver.ble_enable())
    }
}
