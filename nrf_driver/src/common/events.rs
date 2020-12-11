use crate::ffi;
use num_traits::FromPrimitive;
use crate::ffi::{ble_evt_user_mem_release_t, ble_common_evt_t};
use crate::common::types::ConnHandle;
use crate::common::enums::BleMemType;


#[repr(u16)]
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug)]
pub enum CommonEventId {
    MemRequest = ffi::BLE_COMMON_EVTS_BLE_EVT_USER_MEM_REQUEST as u16,
    MemRelease = ffi::BLE_COMMON_EVTS_BLE_EVT_USER_MEM_RELEASE as u16,
}

impl CommonEventId {
    pub fn try_from(id: u16) -> Option<Self> {
        FromPrimitive::from_u16(id)
    }
}


#[derive(Copy, Clone, Debug)]
pub enum CommonEvent {
    MemRequest(CommonEventMemRequest),
    MemRelease(CommonEventMemRelease),
}

impl CommonEvent {
    pub(crate) unsafe fn from_c(id: CommonEventId, e: *const ble_common_evt_t) -> Self {
        match id {
            CommonEventId::MemRequest => CommonEvent::MemRequest(CommonEventMemRequest::from_c((*e).conn_handle, &(*e).params.user_mem_request)),
            CommonEventId::MemRelease => CommonEvent::MemRelease(CommonEventMemRelease::from_c(&(*e).params.user_mem_release))
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct CommonEventMemRequest {
    pub conn_handle: ConnHandle,
    pub mem_type: BleMemType,
}


impl CommonEventMemRequest {
    pub fn id() -> u16 {
        CommonEventId::MemRequest as u16
    }

    unsafe fn from_c(conn_handle: ConnHandle, e: *const ffi::ble_evt_user_mem_request_t) -> Self {
        Self {
            conn_handle,
            mem_type: FromPrimitive::from_u8((*e).type_).unwrap_or(BleMemType::Invalid)
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct CommonEventMemRelease {
    pub mem_type: BleMemType,
    // TODO: Block Data
}


impl CommonEventMemRelease {
    pub fn id() -> u16 {
        CommonEventId::MemRelease as u16
    }

    unsafe fn from_c(e: *const ble_evt_user_mem_release_t) -> Self {
        Self {
            mem_type: FromPrimitive::from_u8((*e).type_).unwrap_or(BleMemType::Invalid)
        }
    }
}
