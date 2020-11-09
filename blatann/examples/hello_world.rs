use std::time::Duration;

use std::sync::Arc;
use nrf_driver::event_publisher::{EventHandler, Subscribable};
use blatann::waitable::Waitable;
use blatann::advertiser::{AdvertisingType, Advertiser};
use blatann::events::AdvertisingTimeoutEvent;
use blatann::device::BleDevice;


fn main() {
    let device_com11 = BleDevice::new("COM11".into(), 1_000_000);
    let device_com13 = BleDevice::new("COM13".into(), 1_000_000);

    device_com11.open().unwrap();
    device_com13.open().unwrap();

    let handler = Arc::new(EventDummy {});
    device_com11.advertiser.on_timeout.subscribe(handler.clone());

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

struct EventDummy {}

impl EventHandler<Advertiser, AdvertisingTimeoutEvent> for EventDummy {
    fn handle(self: Arc<Self>, _sender: Arc<Advertiser>, _event: AdvertisingTimeoutEvent) {
        println!("Adv timeout handled async!")
    }
}
