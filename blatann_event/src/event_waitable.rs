use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{RecvError, RecvTimeoutError};
use std::time::Duration;

use crate::{EventArgs, Subscribable, Subscriber, SubscriberAction, Waitable, AsyncEventHandler};


pub struct EventWaitable<S: 'static, E: Clone + 'static> {
    sender: mpsc::Sender<EventArgs<Arc<S>, E>>,
    receiver: mpsc::Receiver<EventArgs<Arc<S>, E>>,
    callbacks: Mutex<Vec<Box<dyn FnOnce(EventArgs<Arc<S>, E>)>>>
}

impl<S: 'static, E: Clone + 'static> EventWaitable<S, E> {
    pub fn new(event: &dyn Subscribable<S, E>) -> Arc<Self> {
        let (sender, receiver) = mpsc::channel();
        let waitable = Arc::new(Self {
            sender,
            receiver,
            callbacks: Mutex::new(vec![]),
        });

        event.subscribe(waitable.clone());

        return waitable;
    }
}

impl<S: 'static, E: Clone + 'static> Waitable<EventArgs<Arc<S>, E>> for EventWaitable<S, E> {
    fn wait_timeout(&self, timeout: Duration) -> Result<EventArgs<Arc<S>, E>, RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }

    fn wait(&self) -> Result<EventArgs<Arc<S>, E>, RecvError> {
        self.receiver.recv()
    }
}

impl<S: 'static, E: Clone + 'static> AsyncEventHandler<EventArgs<Arc<S>, E>> for EventWaitable<S, E> {

    fn then<F>(&self, f: F)
        where F: 'static + FnOnce(EventArgs<Arc<S>, E>) {

        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(Box::new(f));
    }
}

impl<S: 'static, E: Clone + 'static> Subscriber<S, E> for EventWaitable<S, E> {
    fn handle(self: Arc<Self>, sender: Arc<S>, event: E) -> Option<SubscriberAction> {
        self.sender.send((sender.clone(), event.clone())).unwrap_or_else(|e| {
            error!("Failed to send waitable: {:?}", e);
        });
        // Handled the event, unsubscribe
        let mut callbacks = self.callbacks.lock().unwrap();
        for cb in callbacks.drain(..) {
            (cb)((sender.clone(), event.clone()));
        }

        return Some(SubscriberAction::Unsubscribe);
    }
}


