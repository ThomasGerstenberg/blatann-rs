mod device;

use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use crate::device::{BleDevice, AdvertisingType};


fn main() {
    let device_com11 = BleDevice::new("COM11".into(), 1_000_000);
    let device_com13 = BleDevice::new("COM13".into(), 1_000_000);

    device_com11.open().unwrap();
    device_com13.open().unwrap();

    device_com11.advertiser.start(100_f64, 6, AdvertisingType::NonconnectableUndirected).unwrap();
    device_com13.advertiser.start(50_f64, 30, AdvertisingType::NonconnectableUndirected).unwrap();


    println!("Started advertising!");
    sleep(Duration::from_secs(15));

    device_com11.advertiser.stop().unwrap();
    device_com13.advertiser.stop().unwrap();

    println!("Done");
}
