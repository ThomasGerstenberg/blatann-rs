use std::sync::Arc;

use nrf_driver::driver::NrfDriver;
use nrf_driver::DRIVER_MANAGER;
use nrf_driver::error::NrfError;

use crate::advertiser::Advertiser;

pub struct BleDevice {
    pub port: String,
    pub driver: Arc<NrfDriver>,
    pub advertiser: Arc<Advertiser>,
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
            advertiser,
        }
    }

    pub fn open(&self) -> Result<(), NrfError> {
        self.driver.open().and_then(|_| self.driver.ble_enable())
    }
}
