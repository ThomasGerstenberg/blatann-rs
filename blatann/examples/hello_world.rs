#[macro_use]
extern crate log;

use std::sync::Arc;
use std::time::Duration;

use env_logger;
use env_logger::Env;

use blatann::advertiser::{Advertiser, AdvertisingType};
use blatann::device::BleDevice;
use blatann::events::AdvertisingTimeoutEvent;
use blatann_event::{Subscriber, Subscribable, SubscriberAction, Waitable};

fn configure_log() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
}

fn advertise(device: &BleDevice) {
    let waitable = device.advertiser.start(100_f64, 2, AdvertisingType::NonconnectableUndirected).unwrap();
    waitable.wait_timeout(Duration::from_secs(4)).unwrap();
}

fn main() {
    configure_log();
    let device_com11 = BleDevice::new("COM11".into(), 1_000_000);
    let device_com13 = BleDevice::new("COM13".into(), 1_000_000);

    device_com11.open().unwrap();
    device_com13.open().unwrap();

    let handler = Arc::new(EventDummy {});
    let sub_id = device_com11.advertiser.on_timeout.subscribe(handler.clone());
    info!("Started advertising!");
    advertise(&device_com11);
    advertise(&device_com11);
    advertise(&device_com13);
    advertise(&device_com13);
    device_com11.advertiser.on_timeout.unsubscribe(&sub_id);
    advertise(&device_com11);
}

struct EventDummy {}

impl Subscriber<Advertiser, AdvertisingTimeoutEvent> for EventDummy {
    fn handle(self: Arc<Self>, _sender: Arc<Advertiser>, _event: AdvertisingTimeoutEvent) -> Option<SubscriberAction> {
        info!("Got timeout event!");
        return None;
    }
}
