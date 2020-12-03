use num_traits::FromPrimitive;

use crate::common::types::ConnHandle;
use crate::ffi;

use super::enums::*;
use super::types::*;
use crate::common::enums::BleHciStatus;

#[repr(u16)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum GapEventId {
    Connected = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_CONNECTED as u16,
    Disconnected = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_DISCONNECTED as u16,
    Timeout = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_TIMEOUT as u16,
}

impl GapEventId {
    pub fn try_from(id: u16) -> Option<Self> {
        FromPrimitive::from_u16(id)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GapEvent {
    Connected(GapEventConnected),
    Disconnected(GapEventDisconnected),
    Timeout(GapEventTimeout),
}

impl GapEvent {
    pub(crate) unsafe fn from_c(id: GapEventId, e: *const ffi::ble_gap_evt_t) -> Self {
        let conn_handle = (*e).conn_handle;
        let params = &(*e).params;
        match id {
            GapEventId::Timeout => GapEvent::Timeout(GapEventTimeout::from_c(conn_handle, &params.timeout)),
            GapEventId::Connected => GapEvent::Connected(GapEventConnected::from_c(conn_handle, &params.connected)),
            GapEventId::Disconnected => GapEvent::Disconnected(GapEventDisconnected::from_c(conn_handle, &params.disconnected)),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GapEventConnected {
    pub conn_handle: ConnHandle,
    pub address: BleGapAddress,
    pub role: BleGapRole,
    pub conn_params: BleGapConnParams
}

impl GapEventConnected {
    pub fn id() -> u16 {
        GapEventId::Connected as u16
    }

    unsafe fn from_c(conn_handle: ConnHandle, val: *const ffi::ble_gap_evt_connected_t) -> Self {
        Self {
            conn_handle,
            address: BleGapAddress::from_c(&(*val).peer_addr),
            role: FromPrimitive::from_u8((*val).role).unwrap_or(BleGapRole::Invalid),
            conn_params: BleGapConnParams::from_c(&(*val).conn_params)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GapEventDisconnected {
    pub conn_handle: ConnHandle,
    pub reason: BleHciStatus,
}

impl GapEventDisconnected {
    pub fn id() -> u16 {
        GapEventId::Disconnected as u16
    }

    unsafe fn from_c(conn_handle: ConnHandle, val: *const ffi::ble_gap_evt_disconnected_t) -> Self {
        Self {
            conn_handle,
            reason: FromPrimitive::from_u8((*val).reason).unwrap_or(BleHciStatus::InvalidHciCode)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GapEventTimeout {
    pub conn_handle: ConnHandle,
    pub src: BleGapTimeoutSource,
}

impl GapEventTimeout {
    pub fn id() -> u16 {
        GapEventId::Timeout as u16
    }

    unsafe fn from_c(conn_handle: ConnHandle, val: *const ffi::ble_gap_evt_timeout_t) -> Self {
        Self {
            conn_handle,
            src: FromPrimitive::from_u8((*val).src).unwrap(),
        }
    }
}
