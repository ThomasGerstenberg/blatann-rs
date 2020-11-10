use num_traits::FromPrimitive;

use crate::common::types::ConnHandle;
use crate::ffi;

use super::enums::*;
use super::types::*;

#[repr(u16)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BleGapEventId {
    Timeout = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_TIMEOUT as u16,
}

impl BleGapEventId {
    pub fn try_from(id: u16) -> Option<Self> {
        FromPrimitive::from_u16(id)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BleGapEvent {
    Timeout(BleGapTimeout),
}

impl BleGapEvent {
    pub unsafe fn from_c(id: BleGapEventId, e: *const ffi::ble_gap_evt_t) -> Self {
        match id {
            BleGapEventId::Timeout => BleGapEvent::Timeout(BleGapTimeout::from_c((*e).conn_handle, &(*e).params.timeout)),
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct BleGapTimeout {
    pub conn_handle: ConnHandle,
    pub src: BleGapTimeoutSource,
}

impl BleGapTimeout {
    unsafe fn from_c(conn_handle: ConnHandle, val: *const ffi::ble_gap_evt_timeout_t) -> Self {
        Self {
            conn_handle,
            src: FromPrimitive::from_u8((*val).src).unwrap(),
        }
    }
}
