use crate::common::events::{CommonEvent, CommonEventId};
use crate::ffi::ble_evt_t;
use crate::gap::events::{GapEvent, GapEventId};
use num_traits::ToPrimitive;


#[derive(Copy, Clone, Debug)]
pub struct BleEvent {
    pub id: u16,
    pub data: Option<BleEventData>
}

impl BleEvent {
    pub unsafe fn from_c(e: *const ble_evt_t) -> Self {
        Self {
            id: (*e).header.evt_id,
            data: BleEventData::from_c(e)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BleEventId {
    Common(CommonEventId),
    Gap(GapEventId)
}

impl BleEventId {
    pub fn from_u16(id: u16) -> Option<Self> {
        if let Some(id) = CommonEventId::try_from(id) {
            Some(Self::Common(id))
        } else if let Some(id) = GapEventId::try_from(id) {
            Some(Self::Gap(id))
        } else {
            None
        }
    }
}


impl Into<u16> for BleEventId {
    fn into(self) -> u16 {
        match self {
            BleEventId::Common(x) => ToPrimitive::to_u16(&x),
            BleEventId::Gap(x) => ToPrimitive::to_u16(&x)
        }.unwrap()
    }
}


#[derive(Copy, Clone, Debug)]
pub enum BleEventData {
    Common(CommonEvent),
    Gap(GapEvent),
}


impl BleEventData {
    pub unsafe fn from_c(e: *const ble_evt_t) -> Option<Self> {
        let id = (*e).header.evt_id;

        let event = if let Some(id) = CommonEventId::try_from(id) {
            Some(Self::Common(CommonEvent::from_c(id, &(*e).evt.common_evt)))
        } else if let Some(id) = GapEventId::try_from(id) {
            Some(Self::Gap(GapEvent::from_c(id, &(*e).evt.gap_evt)))
        } else {
            None
        };

        return event;
    }
}