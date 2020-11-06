use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use nrf_driver::DRIVER_MANAGER;
use nrf_driver::nrf_driver::NrfDriver;
use nrf_driver::gap::types::BleGapAdvParams;
use nrf_driver::gap::enums::BleGapAdvertisingType;
use nrf_driver::gap::events::BleGapTimeout;
use nrf_driver::event_publisher::{EventHandler, Publishable};


fn main() {

    let driver_com11 = {
        let mut manager = DRIVER_MANAGER.lock().unwrap();
        manager.create("COM11".into(), 1_000_000, false)
    };

    let driver_com13 = {
        let mut manager = DRIVER_MANAGER.lock().unwrap();
        manager.create("COM13".into(), 1_000_000, false)
    };

    driver_com11.open().unwrap();
    driver_com11.ble_enable().unwrap();
    driver_com13.open().unwrap();
    driver_com13.ble_enable().unwrap();

    let timeout_handler = Arc::new(TimeoutHandler {});
    driver_com11.events.gap_timeout.subscribe(timeout_handler.clone());
    driver_com13.events.gap_timeout.subscribe(timeout_handler.clone());

    let adv_params = BleGapAdvParams::new(100_f64, 6, BleGapAdvertisingType::NonconnectableUndirected);
    driver_com11.ble_gap_adv_start(&adv_params).unwrap();
    let adv_params = BleGapAdvParams::new(50_f64, 3, BleGapAdvertisingType::NonconnectableUndirected);
    driver_com13.ble_gap_adv_start(&adv_params).unwrap();
    println!("Started advertising!");
    sleep(Duration::from_secs(15));
    println!("Stopping advertising");
    driver_com11.ble_gap_adv_stop().unwrap_or_default();
    println!("Done");
    driver_com11.close();
}

struct TimeoutHandler {}

impl EventHandler<NrfDriver, BleGapTimeout> for TimeoutHandler {
    fn handle(&self, sender: &NrfDriver, event: &BleGapTimeout) {
        println!("Got event: {:?}", event);
    }
}