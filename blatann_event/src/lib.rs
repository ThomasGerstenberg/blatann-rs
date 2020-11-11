#[macro_use]
extern crate log;

use std::sync::Arc;
use std::sync::mpsc::{RecvError, RecvTimeoutError};
use std::time::Duration;

use uuid::Uuid;

pub mod publisher;
pub mod event_waitable;

pub use publisher::Publisher;
pub use event_waitable::EventWaitable;

pub enum SubscriberAction {
    Unsubscribe
}

pub trait Subscriber<TSender, TEvent> {
    fn handle(self: Arc<Self>, sender: Arc<TSender>, event: TEvent) -> Option<SubscriberAction>;
}

pub trait Subscribable<TSender, TEvent: Clone> {
    fn name(&self) -> &str;
    fn subscribe(&self, subscriber: Arc<dyn Subscriber<TSender, TEvent>>) -> Uuid;
    fn subscribe_once(&self, subscriber: Arc<dyn Subscriber<TSender, TEvent>>) -> Uuid;
    fn unsubscribe(&self, id: &Uuid);
}

#[derive(Debug, Copy, Clone)]
pub struct EventArgs<S, E: Clone>(S, E);


pub trait Waitable<TSender, TEvent: Clone> {
    fn wait_timeout(&self, timeout: Duration) -> Result<EventArgs<Arc<TSender>, TEvent>, RecvTimeoutError>;
    fn wait(&self) -> Result<EventArgs<Arc<TSender>, TEvent>, RecvError>;
}
