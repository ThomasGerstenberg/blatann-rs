#[macro_use]
extern crate log;

use std::sync::mpsc::{RecvError, RecvTimeoutError};
use std::sync::Arc;
use std::time::Duration;

use uuid::Uuid;

pub mod event_waitable;
pub mod publisher;

pub use event_waitable::EventWaitable;
pub use publisher::Publisher;

pub enum SubscriberAction {
    Unsubscribe,
}

pub trait Subscriber<TSender, TEvent> {
    fn handle(self: Arc<Self>, sender: Arc<TSender>, event: TEvent) -> Option<SubscriberAction>;
}

pub trait Subscribable<TSender, TEvent: Clone> {
    fn name(&self) -> &str;
    fn subscribe(&self, subscriber: Arc<dyn Subscriber<TSender, TEvent>>) -> Uuid;
    fn subscribe_once(&self, subscriber: Arc<dyn Subscriber<TSender, TEvent>>) -> Uuid;
}

pub trait Unsubscribable {
    fn unsubscribe(&self, id: Uuid);
}

pub type EventArgs<S, E> = (S, E);

pub trait Waitable<T> {
    fn wait_timeout(&self, timeout: Duration) -> Result<T, RecvTimeoutError>;
    fn wait(&self) -> Result<T, RecvError>;
}

pub trait AsyncEventHandler<T> {
    fn then<F>(&self, f: F)
    where
        F: 'static + FnOnce(T);
}
