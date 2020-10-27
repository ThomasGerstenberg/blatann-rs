use nrf_driver::DRIVER_MANAGER;
use nrf_driver::gap::types::BleGapAdvParams;
use std::thread::sleep;
use std::time::Duration;
use nrf_driver::gap::enums::BleGapAdvertisingType;

fn main() {

    let driver = {
        let mut manager = DRIVER_MANAGER.lock().unwrap();
        manager.create("COM11".into(), 1_000_000, false)
    };
    let driver2 = {
        let mut manager = DRIVER_MANAGER.lock().unwrap();
        manager.create("COM13".into(), 1_000_000, false)
    };

    driver.open().unwrap();
    driver.ble_enable().unwrap();
    driver2.open().unwrap();
    driver2.ble_enable().unwrap();

    let addr = driver.ble_gap_addr_get().unwrap();
    let addr2 = driver2.ble_gap_addr_get().unwrap();
    println!("Got address!: {}, {}. starting advertising", addr.to_string(), addr2.to_string());

    let adv_params = BleGapAdvParams::new(100_f64, 6, BleGapAdvertisingType::NonconnectableUndirected);
    driver.ble_gap_adv_start(adv_params).unwrap();
    let adv_params = BleGapAdvParams::new(50_f64, 3, BleGapAdvertisingType::NonconnectableUndirected);
    driver2.ble_gap_adv_start(adv_params).unwrap();
    println!("Started advertising!");
    sleep(Duration::from_secs(15));
    println!("Stopping advertising");
    driver.ble_gap_adv_stop().unwrap_or_default();
    println!("Done");
    driver.close();
}
