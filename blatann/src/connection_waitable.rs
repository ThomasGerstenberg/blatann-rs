use crate::peer::Peer;
use blatann_event::{
    AsyncEventHandler, Subscribable, Subscriber, SubscriberAction, Unsubscribable, Waitable,
};
use nrf_driver::driver::NrfDriver;
use nrf_driver::gap::enums::{BleGapRole, BleGapTimeoutSource};
use nrf_driver::gap::events::{GapEventConnected, GapEventTimeout};
use std::cell::RefCell;
use std::sync::mpsc::{RecvError, RecvTimeoutError};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use uuid::Uuid;

pub struct ConnectionWaitable {
    role: BleGapRole,
    peer: Arc<Peer>,
    sender: mpsc::Sender<bool>,
    receiver: mpsc::Receiver<bool>,
    timeout_sub_id: RefCell<Option<Uuid>>,
    connect_sub_id: RefCell<Option<Uuid>>,
    callbacks: Mutex<Vec<Box<dyn FnOnce(Option<Arc<Peer>>)>>>,
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
            callbacks: Mutex::new(vec![]),
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
        self.sender.send(success).unwrap();
        let mut callbacks = self.callbacks.lock().unwrap();

        for cb in callbacks.drain(..) {
            if success {
                (cb)(Some(self.peer.clone()))
            } else {
                (cb)(None)
            }
        }
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

impl AsyncEventHandler<Option<Arc<Peer>>> for ConnectionWaitable {
    fn then<F>(&self, f: F)
    where
        F: 'static + FnOnce(Option<Arc<Peer>>),
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(Box::new(f))
    }
}

impl Subscriber<NrfDriver, GapEventTimeout> for ConnectionWaitable {
    fn handle(
        self: Arc<Self>,
        sender: Arc<NrfDriver>,
        event: GapEventTimeout,
    ) -> Option<SubscriberAction> {
        match (self.role, event.src) {
            (BleGapRole::Peripheral, BleGapTimeoutSource::Advertising)
            | (BleGapRole::Central, BleGapTimeoutSource::Conn) => {
                self.event_received(sender, false)
            }
            _ => {}
        }

        return None;
    }
}

impl Subscriber<NrfDriver, GapEventConnected> for ConnectionWaitable {
    fn handle(
        self: Arc<Self>,
        sender: Arc<NrfDriver>,
        event: GapEventConnected,
    ) -> Option<SubscriberAction> {
        match (self.role, event.role) {
            (BleGapRole::Peripheral, BleGapRole::Peripheral)
            | (BleGapRole::Central, BleGapRole::Central) => self.event_received(sender, true),
            _ => {}
        };

        return None;
    }
}
