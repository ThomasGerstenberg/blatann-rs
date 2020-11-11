use std::sync::{Arc, Mutex, Weak};

use uuid::Uuid;

use crate::{Subscribable, Subscriber, SubscriberAction};

#[derive(Debug, Copy, Clone)]
enum SubscriptionMode {
    Once,
    All,
}

struct EventSubscription<S, E: Clone> {
    id: Uuid,
    handler: Weak<dyn Subscriber<S, E>>,
    mode: SubscriptionMode,
    should_remove: bool,
}

impl<S, E: Clone> EventSubscription<S, E> {
    fn process_event(&mut self, sender: Arc<S>, event: E) {
        // Check if the handler is still a valid reference
        let action = match self.handler.upgrade() {
            // Valid, emit the event and return the action
            Some(handler) => {
                handler.handle(sender.clone(), event)
            }
            // Invalid, needs to be unsubscribed
            None => Some(SubscriberAction::Unsubscribe)
        };

        if let Some(SubscriberAction::Unsubscribe) = action {
            self.should_remove = true;
        }
        if let SubscriptionMode::Once = self.mode {
            self.should_remove = true;
        }
    }
}

pub struct Publisher<TSender, TEvent: Clone>
{
    name: String,
    subscribers: Mutex<Vec<EventSubscription<TSender, TEvent>>>,
}

impl<TSender, TEvent: Clone> Publisher<TSender, TEvent> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            subscribers: Mutex::new(vec![]),
        }
    }

    pub fn dispatch(&self, sender: Arc<TSender>, event: TEvent) {
        let mut cleanup = false;

        let mut subscribers = self.subscribers.lock().unwrap();

        for sub in subscribers.iter_mut() {
            sub.process_event(sender.clone(), event.to_owned());

            if sub.should_remove {
                cleanup = true;
            }
        }

        if cleanup {
            subscribers.retain(|s| { !s.should_remove });
        }
    }

    fn subscribe_impl(&self, handler: Arc<dyn Subscriber<TSender, TEvent>>, mode: SubscriptionMode) -> Uuid {
        let id = Uuid::new_v4();
        let sub = EventSubscription {
            id: id.clone(),
            handler: Arc::downgrade(&handler),
            mode,
            should_remove: false,
        };
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.push(sub);

        return id;
    }
}

impl<TSender, TEvent: Clone> Subscribable<TSender, TEvent> for Publisher<TSender, TEvent> {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn subscribe(&self, handler: Arc<dyn Subscriber<TSender, TEvent>>) -> Uuid {
        return self.subscribe_impl(handler, SubscriptionMode::All);
    }

    fn subscribe_once(&self, handler: Arc<dyn Subscriber<TSender, TEvent>>) -> Uuid {
        return self.subscribe_impl(handler, SubscriptionMode::Once);
    }

    fn unsubscribe(&self, id: &Uuid) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.retain(|s| { s.id != *id });
    }
}
