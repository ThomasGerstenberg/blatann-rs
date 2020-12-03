use nrf_driver::gap::enums::{BleGapRole, BleGapTimeoutSource};
use crate::peer::Peer;
use std::sync::{Arc, mpsc};
use nrf_driver::driver::NrfDriver;
use blatann_event::{Subscriber, SubscriberAction, Waitable, Subscribable, Unsubscribable};
use nrf_driver::gap::events::{GapEventTimeout, GapEventConnected};
use std::sync::mpsc::{RecvError, RecvTimeoutError};
use bitflags::_core::time::Duration;
use bitflags::_core::cell::RefCell;
use uuid::Uuid;

pub struct ConnectionWaitable {
    role: BleGapRole,
    peer: Arc<Peer>,
    sender: mpsc::Sender<bool>,
    receiver: mpsc::Receiver<bool>,
    timeout_sub_id: RefCell<Option<Uuid>>,
    connect_sub_id: RefCell<Option<Uuid>>
}


impl ConnectionWaitable {
    pub(crate) fn new(driver: Arc<NrfDriver>, peer: Arc<Peer>, role: BleGapRole) -> Arc<Self> {
        let (sender, receiver) = mpsc::channel();
        let waitable = Arc::new(Self {
            role,
            peer,
            sender,
            receiver,
            timeout_sub_id: RefCell::new(None),
            connect_sub_id: RefCell::new(None),
        });
        let connected_uuid = driver.events.connected.subscribe(waitable.clone());
        let timeout_uuid = driver.events.gap_timeout.subscribe(waitable.clone());
        waitable.timeout_sub_id.replace(Some(timeout_uuid));
        waitable.connect_sub_id.replace(Some(connected_uuid));

        return waitable;
    }

    fn event_received(&self, driver: Arc<NrfDriver>, success: bool) {
        let sub_id = self.timeout_sub_id.borrow();
        if let Some(id) = *sub_id {
            driver.events.gap_timeout.unsubscribe(id)
        }
        let sub_id = self.connect_sub_id.borrow();
        if let Some(id) = *sub_id {
            driver.events.connected.unsubscribe(id)
        }
        self.sender.send(success).unwrap()
    }
}

impl Waitable<Option<Arc<Peer>>> for ConnectionWaitable {
    fn wait_timeout(&self, timeout: Duration) -> Result<Option<Arc<Peer>>, RecvTimeoutError> {
        self.receiver.recv_timeout(timeout).and_then(|result| {

            if result {
                Ok(Some(self.peer.clone()))
            } else {
                Ok(None)
            }
        })
    }

    fn wait(&self) -> Result<Option<Arc<Peer>>, RecvError> {
        self.receiver.recv().and_then(|result| {
            if result {
                Ok(Some(self.peer.clone()))
            } else {
                Ok(None)
            }
        })
    }
}


impl Subscriber<NrfDriver, GapEventTimeout> for ConnectionWaitable {
    fn handle(self: Arc<Self>, sender: Arc<NrfDriver>, event: GapEventTimeout) -> Option<SubscriberAction> {
        match (self.role, event.src) {
            (BleGapRole::Peripheral, BleGapTimeoutSource::Advertising) |
            (BleGapRole::Central, BleGapTimeoutSource::Conn) => self.event_received(sender, false),
            _ => {}
        }

        return None;
    }
}

impl Subscriber<NrfDriver, GapEventConnected> for ConnectionWaitable {
    fn handle(self: Arc<Self>, sender: Arc<NrfDriver>, event: GapEventConnected) -> Option<SubscriberAction> {
        match (self.role, event.role) {
            (BleGapRole::Peripheral, BleGapRole::Peripheral) |
            (BleGapRole::Central, BleGapRole::Central) => self.event_received(sender, true),
            _ => {}
        };

        return None;
    }
}
