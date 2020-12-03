use std::sync::Arc;

use blatann_event::{Publisher, Unsubscribable, Subscribable, Subscriber};
use uuid::Uuid;

use crate::ble_event::*;
use crate::common::events::*;
use crate::driver::NrfDriver;
use crate::gap::events::*;


pub struct NrfEventPublisher<TEvent: Clone> {
    id: BleEventId,
    publisher: Publisher<NrfDriver, TEvent>
}

impl<TEvent: Clone> NrfEventPublisher<TEvent> {
    pub fn new(name: &str, id: BleEventId) -> Self {
        Self {
            id,
            publisher: Publisher::new(name)
        }
    }

    pub fn id(&self) -> BleEventId {
        self.id
    }

    fn dispatch(&self, sender: Arc<NrfDriver>, event: TEvent) {
        self.publisher.dispatch(sender, event)
    }
}

impl<TEvent: Clone> Subscribable<NrfDriver, TEvent> for NrfEventPublisher<TEvent> {
    fn name(&self) -> &str {
        self.publisher.name()
    }

    fn subscribe(&self, subscriber: Arc<dyn Subscriber<NrfDriver, TEvent>>) -> Uuid {
        self.publisher.subscribe(subscriber)
    }

    fn subscribe_once(&self, subscriber: Arc<dyn Subscriber<NrfDriver, TEvent>>) -> Uuid {
        self.publisher.subscribe_once(subscriber)
    }
}

impl<TEvent: Clone> Unsubscribable for NrfEventPublisher<TEvent> {
    fn unsubscribe(&self, id: Uuid) {
        self.publisher.unsubscribe(id)
    }
}


#[allow(dead_code)]
pub struct NrfDriverEvents {
    pub user_mem_request: NrfEventPublisher<CommonEventMemRequest>,
    pub user_mem_release: NrfEventPublisher<CommonEventMemRelease>,
    pub gap_timeout: NrfEventPublisher<GapEventTimeout>,
    pub connected: NrfEventPublisher<GapEventConnected>,
    pub disconnected: NrfEventPublisher<GapEventDisconnected>,
}

impl NrfDriverEvents {
    pub(crate) fn new() -> Self {
        Self {
            user_mem_request: NrfEventPublisher::new("User Mem Request", BleEventId::Common(CommonEventId::MemRequest)),
            user_mem_release: NrfEventPublisher::new("User Mem Release", BleEventId::Common(CommonEventId::MemRelease)),
            gap_timeout: NrfEventPublisher::new("Gap Timeout", BleEventId::Gap(GapEventId::Timeout)),
            connected: NrfEventPublisher::new("Connected", BleEventId::Gap(GapEventId::Connected)),
            disconnected: NrfEventPublisher::new("Disconnected", BleEventId::Gap(GapEventId::Disconnected)),
        }
    }

    pub(crate) fn dispatch(&self, driver: Arc<NrfDriver>, ble_event: BleEventData) {
        match ble_event {
            BleEventData::Common(sub_event) => match sub_event {
                CommonEvent::MemRequest(e) => self.user_mem_request.dispatch(driver, e),
                CommonEvent::MemRelease(e) => self.user_mem_release.dispatch(driver, e),
            }
            BleEventData::Gap(sub_event) => match sub_event {
                GapEvent::Timeout(e) => self.gap_timeout.dispatch(driver, e),
                GapEvent::Connected(e) => self.connected.dispatch(driver, e),
                GapEvent::Disconnected(e) => self.disconnected.dispatch(driver, e)
            }
        };
    }

    pub(crate) fn unsubscribe(&self, event_id: BleEventId, subscription_id: Uuid) {
        let publisher: &dyn Unsubscribable = match event_id {
            BleEventId::Common(sub_id) => match sub_id {
                CommonEventId::MemRequest => &self.user_mem_request,
                CommonEventId::MemRelease => &self.user_mem_release,
            }
            BleEventId::Gap(sub_id) => match sub_id {
                GapEventId::Timeout => &self.gap_timeout,
                GapEventId::Connected => &self.connected,
                GapEventId::Disconnected => &self.disconnected,
            }
        };
        publisher.unsubscribe(subscription_id)
    }
}
