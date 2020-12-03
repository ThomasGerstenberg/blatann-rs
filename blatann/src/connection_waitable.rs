use nrf_driver::gap::enums::{BleGapRole, BleGapTimeoutSource};
use crate::peer::Peer;
use std::sync::Arc;
use nrf_driver::driver::NrfDriver;
use blatann_event::{Subscriber, SubscriberAction, Waitable, EventArgs};
use nrf_driver::gap::events::{GapEventTimeout, GapEventConnected};
use std::sync::mpsc::{RecvError, RecvTimeoutError};
use bitflags::_core::time::Duration;

pub struct ConnectionWaitable {
    role: BleGapRole,
    peer: Arc<Peer>,
    driver: Arc<NrfDriver>
}


impl ConnectionWaitable {
    pub(crate) fn new(driver: Arc<NrfDriver>, peer: Arc<Peer>, role: BleGapRole) -> Arc<Self> {
        let waitable = Arc::new(Self {
            role,
            peer,
            driver
        });
    }
}

impl Waitable<Peer, bool> for ConnectionWaitable {
    fn wait_timeout(&self, timeout: Duration) -> Result<EventArgs<Arc<Peer>, bool>, RecvTimeoutError> {
        unimplemented!()
    }

    fn wait(&self) -> Result<EventArgs<Arc<Peer>, bool>, RecvError> {
        unimplemented!()
    }
}


impl Subscriber<NrfDriver, GapEventTimeout> for ConnectionWaitable {
    fn handle(self: Arc<Self>, sender: Arc<NrfDriver>, event: GapEventTimeout) -> Option<SubscriberAction> {
        let expected_source = if let BleGapRole::Peripheral = self.role {
            BleGapTimeoutSource::Advertising
        } else {
            BleGapTimeoutSource::Conn
        };
        if event.src == expected_source {

        };
        return Some(SubscriberAction::Unsubscribe)
    }
}

impl Subscriber<NrfDriver, GapEventConnected> for ConnectionWaitable {
    fn handle(self: Arc<Self>, sender: Arc<NrfDriver>, event: GapEventConnected) -> Option<SubscriberAction> {
        unimplemented!()
    }
}