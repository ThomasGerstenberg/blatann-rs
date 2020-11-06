use crate::common::events::{BleCommonEvent, BleCommonEventId};
use crate::gap::events::{BleGapEvent, BleGapEventId};
use crate::ffi::ble_evt_t;


#[derive(Copy, Clone, Debug)]
pub enum BleEvent {
    Common(BleCommonEvent),
    Gap(BleGapEvent)
}


impl BleEvent {
    pub unsafe fn from_c(e: *const ble_evt_t) -> Option<Self> {
        let id = (*e).header.evt_id;

        let event = if let Some(id) = BleCommonEventId::try_from(id) {
            Some(Self::Common(BleCommonEvent::from_c(id, &(*e).evt.common_evt)))
        }
        else if let Some(id) = BleGapEventId::try_from(id) {
            Some(Self::Gap(BleGapEvent::from_c(id, &(*e).evt.gap_evt)))
        }
        else {
            None
        };

        return event;
    }
}