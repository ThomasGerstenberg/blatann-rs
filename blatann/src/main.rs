pub mod device;
pub mod event_waitable;
mod waitable;

use std::time::Duration;

use crate::device::{BleDevice, AdvertisingType};
use crate::waitable::Waitable;


fn main() {
    let device_com11 = BleDevice::new("COM11".into(), 1_000_000);
    let device_com13 = BleDevice::new("COM13".into(), 1_000_000);

    device_com11.open().unwrap();
    device_com13.open().unwrap();

    let waitable_11 = device_com11.advertiser.start(100_f64, 6, AdvertisingType::NonconnectableUndirected).unwrap();
    let waitable_13 = device_com13.advertiser.start(50_f64, 6, AdvertisingType::NonconnectableUndirected).unwrap();

    println!("Started advertising!");
    let result_11 = waitable_11.wait_timeout(Duration::from_secs(20));
    println!("Waitable done!");
    result_11.unwrap();
    println!("Waiting for 13");
    waitable_13.wait_timeout(Duration::from_secs(20)).unwrap();

    println!("Done");
}
