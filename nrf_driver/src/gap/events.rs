use crate::driver;
use crate::common::types::ConnHandle;
use super::types::*;
use super::enums::*;
use num_traits::FromPrimitive;

#[repr(u16)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BleGapEventId {
    Timeout = driver::BLE_GAP_EVTS_BLE_GAP_EVT_TIMEOUT as u16,
}

impl BleGapEventId {
    pub fn try_from(id: u16) -> Option<Self> {
        FromPrimitive::from_u16(id)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BleGapEvent {
    Timeout(Timeout),
}

impl BleGapEvent {
    pub unsafe fn from_c(id: BleGapEventId, e: *const driver::ble_gap_evt_t) -> Self {
         match id {
            BleGapEventId::Timeout => BleGapEvent::Timeout(Timeout::from_c((*e).conn_handle, &(*e).params.timeout)),
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Timeout {
    pub conn_handle: ConnHandle,
    pub src: BleGapTimeoutSource
}

impl Timeout {
    unsafe fn from_c(conn_handle: ConnHandle, val: *const driver::ble_gap_evt_timeout_t) -> Self {
        Self {
            conn_handle,
            src: FromPrimitive::from_u8((*val).src).unwrap()
        }
    }
}
