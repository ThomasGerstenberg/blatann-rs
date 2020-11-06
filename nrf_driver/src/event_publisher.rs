use std::sync::{Mutex, Weak, Arc};

pub trait EventHandler<TSender, TEvent> {
    fn handle(&self, sender: &TSender, event: &TEvent);
}

pub trait Publishable<TSender, TEvent> {
    fn subscribe(&self, handler: Arc<dyn EventHandler<TSender, TEvent>>);
}

pub struct EventPublisher<TSender, TEvent>
{
    subscribers: Mutex<Vec<Weak<dyn EventHandler<TSender, TEvent>>>>,
}

impl<TSender, TEvent> EventPublisher<TSender, TEvent> {
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(vec![]),
        }
    }

    pub fn dispatch(&self, sender: &TSender, event: &TEvent) {
        let mut cleanup = false;

        let mut subscribers = self.subscribers.lock().unwrap();
        for sub in subscribers.iter() {
            if let Some(s) = sub.upgrade() {
                s.handle(sender, event)
            } else {
                cleanup = true;
            }
        }

        if cleanup {
            subscribers.retain(|ref s| {
                match s.clone().upgrade() {
                    Some(_) => true,
                    None => false,
                }
            });
        }
    }
}

impl<TSender, TEvent> Publishable<TSender, TEvent> for EventPublisher<TSender, TEvent> {
    fn subscribe(&self, handler: Arc<dyn EventHandler<TSender, TEvent>>) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.push(Arc::downgrade(&handler));
    }
}
