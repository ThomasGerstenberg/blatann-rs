use std::sync::{Arc, Mutex, Weak};

pub trait EventHandler<TSender, TEvent> {
    fn handle(self: Arc<Self>, sender: Arc<TSender>, event: TEvent);
}

pub trait Subscribable<TSender, TEvent: Clone> {
    fn subscribe(&self, handler: Arc<dyn EventHandler<TSender, TEvent>>);
}

pub struct EventPublisher<TSender, TEvent: Clone>
{
    subscribers: Mutex<Vec<Weak<dyn EventHandler<TSender, TEvent>>>>,
}

impl<TSender, TEvent: Clone> EventPublisher<TSender, TEvent> {
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(vec![]),
        }
    }

    pub fn dispatch(&self, sender: Arc<TSender>, event: TEvent) {
        let mut cleanup = false;

        let mut subscribers = self.subscribers.lock().unwrap();
        for sub in subscribers.iter() {
            if let Some(s) = sub.upgrade() {
                s.handle(sender.clone(), event.clone())
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

impl<TSender, TEvent: Clone> Subscribable<TSender, TEvent> for EventPublisher<TSender, TEvent> {
    fn subscribe(&self, handler: Arc<dyn EventHandler<TSender, TEvent>>) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.push(Arc::downgrade(&handler));
    }
}
