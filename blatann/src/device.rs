use nrf_driver::driver::NrfDriver;
use nrf_driver::manager::NrfDriverManager;
use nrf_driver::DRIVER_MANAGER;
use std::sync::Arc;
use nrf_driver::event_publisher::{EventHandler, Publishable};
use nrf_driver::gap::events::BleGapTimeout;
use nrf_driver::gap::enums::BleGapAdvertisingType;
use nrf_driver::utils::Milliseconds;
use nrf_driver::gap::types::BleGapAdvParams;
use nrf_driver::error::{NrfError, NrfErrorType};

pub type AdvertisingType = BleGapAdvertisingType;

pub struct Advertiser {
    driver: Arc<NrfDriver>
}

impl Advertiser {
    pub fn new(driver: Arc<NrfDriver>) -> Self {
        Self { driver }
    }

    pub fn start(&self, interval: Milliseconds, timeout_s: u16, adv_type: AdvertisingType) -> Result<(), NrfError> {
        let params = BleGapAdvParams::new(interval, timeout_s, adv_type);

        self.driver.ble_gap_adv_start(&params)
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
    fn handle(&self, _sender: &NrfDriver, event: &BleGapTimeout) {
        println!("Got timeout event");
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
        let advertiser = Arc::new(Advertiser::new(driver.clone()));
        driver.events.gap_timeout.subscribe(advertiser.clone());

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