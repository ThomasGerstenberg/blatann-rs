use std::sync::{mpsc, Arc};
use nrf_driver::event_publisher::{EventHandler, Subscribable};
use nrf_driver::error::NrfError;
use std::time::Duration;
use std::sync::mpsc::{RecvTimeoutError, RecvError};
use crate::waitable::Waitable;

#[derive(Debug, Copy, Clone)]
pub struct EventArgs<S, E: Clone>(S, E);

pub type EventWaitableResult<S, E> = Result<Arc<EventWaitable<S, E>>, NrfError>;

pub struct EventWaitable<TSender: 'static, TEvent: Clone+'static> {
    sender: mpsc::Sender<EventArgs<Arc<TSender>, TEvent>>,
    receiver: mpsc::Receiver<EventArgs<Arc<TSender>, TEvent>>,
}

impl<TSender: 'static, TEvent: Clone+'static> EventWaitable<TSender, TEvent> {
    pub fn new(event: &dyn Subscribable<TSender, TEvent>) -> Arc<Self> {
        let (sender, receiver) = mpsc::channel();
        let waitable = Arc::new(Self {
            sender,
            receiver
        });
        event.subscribe(waitable.clone());

        return waitable;
    }
}

impl<TSender: 'static, TEvent: Clone+'static> Waitable<TSender, TEvent> for EventWaitable<TSender, TEvent> {
    fn wait_timeout(&self, timeout: Duration) -> Result<EventArgs<Arc<TSender>, TEvent>, RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }

    fn wait(&self) -> Result<EventArgs<Arc<TSender>, TEvent>, RecvError> {
        self.receiver.recv()
    }
}

impl<TSender: 'static, TEvent: Clone+'static> EventHandler<TSender, TEvent> for EventWaitable<TSender, TEvent> {
    fn handle(self: Arc<Self>, sender: Arc<TSender>, event: TEvent) {
        self.sender.send(EventArgs(sender.clone(), event.clone())).unwrap_or_else(|_| {
            println!("Failed to send waitable");
        });
    }
}
