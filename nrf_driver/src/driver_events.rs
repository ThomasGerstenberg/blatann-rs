use std::sync::Arc;

use blatann_event::{Publisher, Unsubscribable, Subscribable, Subscriber};
use uuid::Uuid;

use crate::ble_event::*;
use crate::common::events::*;
use crate::driver::NrfDriver;
use crate::gap::events::*;
use std::collections::HashMap;


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
    pub connected: NrfEventPublisher<GapEventConnected>,
    pub disconnected: NrfEventPublisher<GapEventDisconnected>,
    pub gap_timeout: NrfEventPublisher<GapEventTimeout>,
    pub phy_update_request: NrfEventPublisher<GapEventPhyUpdateRequest>,
    pub phy_update: NrfEventPublisher<GapEventPhyUpdate>,
    pub data_length_update_request: NrfEventPublisher<GapEventDataLengthUpdateRequest>,
    pub data_length_update: NrfEventPublisher<GapEventDataLengthUpdate>,
}

impl NrfDriverEvents {
    pub(crate) fn new() -> Self {
        Self {
            // Common
            user_mem_request: NrfEventPublisher::new("User Mem Request", BleEventId::Common(CommonEventId::MemRequest)),
            user_mem_release: NrfEventPublisher::new("User Mem Release", BleEventId::Common(CommonEventId::MemRelease)),
            // Gap
            connected: NrfEventPublisher::new("Connected", BleEventId::Gap(GapEventId::Connected)),
            disconnected: NrfEventPublisher::new("Disconnected", BleEventId::Gap(GapEventId::Disconnected)),
            gap_timeout: NrfEventPublisher::new("Gap Timeout", BleEventId::Gap(GapEventId::Timeout)),
            phy_update_request: NrfEventPublisher::new("Phy Update Request", BleEventId::Gap(GapEventId::PhyUpdateRequest)),
            phy_update: NrfEventPublisher::new("Phy Update", BleEventId::Gap(GapEventId::PhyUpdate)),
            data_length_update_request: NrfEventPublisher::new("Data Length Update Request", BleEventId::Gap(GapEventId::DataLengthUpdateRequest)),
            data_length_update: NrfEventPublisher::new("Data Length Update", BleEventId::Gap(GapEventId::DataLengthUpdate)),
        }
    }

    fn events(&self) -> Vec<(BleEventId, &dyn Unsubscribable)> {
        vec![
            (self.user_mem_request.id, &self.user_mem_request),
            (self.user_mem_release.id, &self.user_mem_release),
            (self.gap_timeout.id, &self.gap_timeout),
            (self.connected.id, &self.connected),
            (self.disconnected.id, &self.disconnected),
            (self.gap_timeout.id, &self.gap_timeout),
            (self.phy_update_request.id, &self.phy_update_request),
            (self.phy_update.id, &self.phy_update),
            (self.data_length_update_request.id, &self.data_length_update_request),
            (self.data_length_update.id, &self.data_length_update)
        ]
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
                GapEvent::Disconnected(e) => self.disconnected.dispatch(driver, e),
                GapEvent::PhyUpdateRequest(e) => self.phy_update_request.dispatch(driver, e),
                GapEvent::PhyUpdate(e) => self.phy_update.dispatch(driver, e),
                GapEvent::DataLengthUpdateRequest(e) => self.data_length_update_request.dispatch(driver, e),
                GapEvent::DataLengthUpdate(e) => self.data_length_update.dispatch(driver, e),
            }
        };
    }

    pub(crate) fn unsubscribe(&self, event_id: BleEventId, subscription_id: Uuid) {
        let event_map = self.events()
            .iter()
            .cloned()
            .map(|(k, v)| { (k.into(), v) })
            .collect::<HashMap<u16, &dyn Unsubscribable>>();

        let event_val = event_id.into();
        match event_map.get(&event_val) {
            None => warn!("Unknown event id {}!", event_val),
            Some(p) => p.unsubscribe(subscription_id)
        }
    }
}
