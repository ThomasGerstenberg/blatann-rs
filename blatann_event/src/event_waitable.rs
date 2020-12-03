use std::cell::RefCell;
use std::sync::{Arc, mpsc};
use std::sync::mpsc::{RecvError, RecvTimeoutError};
use std::time::Duration;

use uuid::Uuid;

use crate::{EventArgs, Subscribable, Subscriber, SubscriberAction, Waitable};

pub struct EventWaitable<TSender: 'static, TEvent: Clone + 'static> {
    subscription_id: RefCell<Option<Uuid>>,
    sender: mpsc::Sender<EventArgs<Arc<TSender>, TEvent>>,
    receiver: mpsc::Receiver<EventArgs<Arc<TSender>, TEvent>>,
}

impl<TSender: 'static, TEvent: Clone + 'static> EventWaitable<TSender, TEvent> {
    pub fn new(event: &dyn Subscribable<TSender, TEvent>) -> Arc<Self> {
        let (sender, receiver) = mpsc::channel();
        let waitable = Arc::new(Self {
            sender,
            receiver,
            subscription_id: RefCell::new(None),
        });

        let sub_id = event.subscribe(waitable.clone());
        waitable.subscription_id.replace(Some(sub_id));

        return waitable;
    }
}

impl<TSender: 'static, TEvent: Clone + 'static> Waitable<EventArgs<Arc<TSender>, TEvent>> for EventWaitable<TSender, TEvent> {
    fn wait_timeout(&self, timeout: Duration) -> Result<EventArgs<Arc<TSender>, TEvent>, RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }

    fn wait(&self) -> Result<EventArgs<Arc<TSender>, TEvent>, RecvError> {
        self.receiver.recv()
    }
}

impl<TSender: 'static, TEvent: Clone + 'static> Subscriber<TSender, TEvent> for EventWaitable<TSender, TEvent> {
    fn handle(self: Arc<Self>, sender: Arc<TSender>, event: TEvent) -> Option<SubscriberAction> {
        self.sender.send(EventArgs(sender.clone(), event.clone())).unwrap_or_else(|e| {
            error!("Failed to send waitable: {:?}", e);
        });
        // Handled the event, unsubscribe
        return Some(SubscriberAction::Unsubscribe);
    }
}
