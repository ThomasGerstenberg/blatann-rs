use std::time::Duration;
use crate::event_waitable::EventArgs;
use std::sync::Arc;
use std::sync::mpsc::{RecvTimeoutError, RecvError};


pub trait Waitable<TSender, TEvent: Clone> {
    fn wait_timeout(&self, timeout: Duration) -> Result<EventArgs<Arc<TSender>, TEvent>, RecvTimeoutError>;
    fn wait(&self) -> Result<EventArgs<Arc<TSender>, TEvent>, RecvError>;
}
