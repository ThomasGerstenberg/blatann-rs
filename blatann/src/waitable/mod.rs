use std::sync::Arc;
use std::sync::mpsc::{RecvError, RecvTimeoutError};
use std::time::Duration;

use event_waitable::EventArgs;

pub mod event_waitable;


pub trait Waitable<TSender, TEvent: Clone> {
    fn wait_timeout(&self, timeout: Duration) -> Result<EventArgs<Arc<TSender>, TEvent>, RecvTimeoutError>;
    fn wait(&self) -> Result<EventArgs<Arc<TSender>, TEvent>, RecvError>;
}
