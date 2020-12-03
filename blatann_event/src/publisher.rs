use std::sync::{Arc, Mutex, Weak};

use uuid::Uuid;

use crate::{Subscribable, Subscriber, SubscriberAction, Unsubscribable};
use std::sync::atomic::AtomicBool;

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

impl<S, E: Clone> Clone for EventSubscription<S, E> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            handler: self.handler.clone(),
            mode: self.mode.clone(),
            should_remove: self.should_remove,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.id = source.id.clone();
        self.handler = source.handler.clone();
        self.mode = source.mode.clone();
        self.should_remove = source.should_remove;
    }
}

pub struct Publisher<TSender, TEvent: Clone> {
    name: String,
    subscribers: Mutex<Vec<EventSubscription<TSender, TEvent>>>,
    dispatch_lock: Mutex<()>
}

impl<TSender, TEvent: Clone> Publisher<TSender, TEvent> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            subscribers: Mutex::new(vec![]),
            dispatch_lock: Mutex::new(()),
        }
    }

    pub fn dispatch(&self, sender: Arc<TSender>, event: TEvent) {
        let _dispatch_lock = self.dispatch_lock.lock().unwrap();

        // Get a clone of the list to iterate mutex-free
        let mut temp_subs = vec![];
        {
            let subs = self.subscribers.lock().unwrap();
            for s in subs.iter() {
                temp_subs.push(s.clone())
            }
        };

        for sub in temp_subs.iter_mut() {
            sub.process_event(sender.clone(), event.to_owned());
        }

        // For each of the subs that need cleanup, remove from the mutex-locked list
        temp_subs.retain(|s| { s.should_remove });
        {
            let mut subscribers = self.subscribers.lock().unwrap();
            for sub in temp_subs.iter() {
                subscribers.retain(|s| s.id != sub.id)
            }
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

}

impl<TSender, TEvent: Clone> Unsubscribable for Publisher<TSender, TEvent> {
    fn unsubscribe(&self, id: Uuid) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.retain(|s| { s.id != id });
    }
}