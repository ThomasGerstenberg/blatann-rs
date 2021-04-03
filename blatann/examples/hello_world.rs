#[macro_use]
extern crate log;

use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use blatann_event::{Subscribable, Subscriber, SubscriberAction, Waitable, AsyncEventHandler};
use env_logger;
use env_logger::Env;

use blatann::advertise_data::{AdvData, AdvDataType};
use blatann::advertiser::{Advertiser, AdvType};
use blatann::device::BleDevice;
use blatann::events::{AdvertisingTimeoutEvent, ConnectionEvent};
use blatann::peer::Peer;

fn configure_log() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
}


fn main() {
    configure_log();
    let device = BleDevice::new("COM13".into(), 1_000_000);

    device.open().unwrap();

    let handler = Arc::new(EventDummy {});
    device.advertiser.on_timeout.subscribe(handler);

    let mut adv_data = AdvData::default();
    adv_data.add_entry(AdvDataType::CompleteLocalName as u8, b"Blatann-rs!!");
    device.advertiser.set_params(50_f64, 50, AdvType::ConnectableUndirected, false);
    device.advertiser.set_data(Some(&adv_data), None).unwrap();

    info!("Started advertising!");
    let connect_waitable = device.advertiser.start().unwrap();

    connect_waitable.then(|peer| {
       if let Some(_) = peer {
           info!("then(): got peer!");
       }
    });

    let result = connect_waitable.wait().unwrap();
    info!("Got Peer: {:?}", result.is_some());
    sleep(Duration::from_secs(10));
    if let Some(peer) = result {
        info!("Disconnecting...");
        let (_, event) = peer.disconnect().unwrap().wait().unwrap();
        info!("{:?}", event);
    }
    info!("Done!")
}

struct EventDummy {}

impl Subscriber<Peer, ConnectionEvent> for EventDummy {
    fn handle(self: Arc<Self>, _sender: Arc<Peer>, _event: ConnectionEvent) -> Option<SubscriberAction> {
        info!("Peer connected!");
        return None;
    }
}

impl Subscriber<Advertiser, AdvertisingTimeoutEvent> for EventDummy {
    fn handle(self: Arc<Self>, sender: Arc<Advertiser>, event: AdvertisingTimeoutEvent) -> Option<SubscriberAction> {
        info!("Got advertising timeout!");
        return None
    }
}