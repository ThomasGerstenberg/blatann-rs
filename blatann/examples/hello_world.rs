#[macro_use]
extern crate log;

use std::sync::Arc;
use std::time::Duration;

use env_logger;
use env_logger::Env;

use blatann::advertiser::{Advertiser, AdvType};
use blatann::device::BleDevice;
use blatann::events::AdvertisingTimeoutEvent;
use blatann_event::{Subscriber, Subscribable, SubscriberAction, Waitable};
use blatann::advertise_data::{AdvData, AdvDataType};

fn configure_log() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
}


fn main() {
    configure_log();
    let device_com11 = BleDevice::new("COM11".into(), 1_000_000);
    let device_com13 = BleDevice::new("COM13".into(), 1_000_000);

    device_com11.open().unwrap();
    device_com13.open().unwrap();

    let handler = Arc::new(EventDummy {});
    device_com11.advertiser.on_timeout.subscribe(handler.clone());
    info!("Started advertising!");
    let mut adv_data = AdvData::default();
    adv_data.add_entry(AdvDataType::CompleteLocalName as u8, "Blatann-rs!!".as_bytes());
    device_com11.advertiser.set_params(100_f64, 5, AdvType::NonconnectableUndirected, true);
    device_com13.advertiser.set_params(50_f64, 25, AdvType::NonconnectableUndirected, false);
    device_com11.advertiser.set_data(Some(&adv_data), None).unwrap();
    device_com13.advertiser.set_data(Some(&adv_data), None).unwrap();

    let waitable1 = device_com11.advertiser.start().unwrap();
    let waitable2 = device_com13.advertiser.start().unwrap();
    info!("Waiting for COM11");
    waitable1.wait().unwrap();
    info!("Waiting for COM13");
    waitable2.wait().unwrap();
    info!("Done!")
}

struct EventDummy {}

impl Subscriber<Advertiser, AdvertisingTimeoutEvent> for EventDummy {
    fn handle(self: Arc<Self>, _sender: Arc<Advertiser>, _event: AdvertisingTimeoutEvent) -> Option<SubscriberAction> {
        info!("Got timeout event!");
        return None;
    }
}
