use crate::driver;
use num_traits::FromPrimitive;
use crate::driver::{ble_evt_user_mem_release_t, ble_common_evt_t};


#[repr(u16)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BleCommonEventId {
    MemRequest = driver::BLE_COMMON_EVTS_BLE_EVT_USER_MEM_REQUEST as u16,
    MemRelease = driver::BLE_COMMON_EVTS_BLE_EVT_USER_MEM_RELEASE as u16,
}

impl BleCommonEventId {
    pub fn try_from(id: u16) -> Option<Self> {
        FromPrimitive::from_u16(id)
    }
}


#[derive(Copy, Clone, Debug)]
pub enum BleCommonEvent {
    MemRequest(MemRequest),
    MemRelease(MemRelease),
}

impl BleCommonEvent {
    pub unsafe fn from_c(id: BleCommonEventId, e: *const ble_common_evt_t) -> Self {
        match id {
            BleCommonEventId::MemRequest => BleCommonEvent::MemRequest(MemRequest::from_c(&(*e).params.user_mem_request)),
            BleCommonEventId::MemRelease => BleCommonEvent::MemRelease(MemRelease::from_c(&(*e).params.user_mem_release))
        }
    }
}


#[repr(u8)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BleMemType {
    Invalid = driver::BLE_USER_MEM_TYPE_INVALID as u8,
    GattsQueuedWrites = driver::BLE_USER_MEM_TYPE_GATTS_QUEUED_WRITES as u8,
}


#[derive(Debug, Copy, Clone)]
pub struct MemRequest {
    pub mem_type: BleMemType
}


impl MemRequest {
    unsafe fn from_c(e: *const driver::ble_evt_user_mem_request_t) -> Self {
        Self {
            mem_type: FromPrimitive::from_u8((*e).type_).unwrap_or(BleMemType::Invalid)
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct MemRelease {
    pub mem_type: BleMemType,
    // TODO: Block Data
}


impl MemRelease {
    unsafe fn from_c(e: *const ble_evt_user_mem_release_t) -> Self {
        Self {
            mem_type: FromPrimitive::from_u8((*e).type_).unwrap_or(BleMemType::Invalid)
        }
    }
}
