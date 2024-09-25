use num_traits::FromPrimitive;

use crate::ble_event::{BleEventDataType, BleEventId, CommonEventId};
use crate::common::enums::BleMemType;
use crate::common::types::ConnHandle;
use crate::ffi;
use crate::ffi::ble_evt_user_mem_release_t;

#[derive(Debug, Copy, Clone)]
pub struct CommonEventMemRequest {
    pub conn_handle: ConnHandle,
    pub mem_type: BleMemType,
}

impl CommonEventMemRequest {
    pub(crate) unsafe fn from_c(
        conn_handle: ConnHandle,
        e: *const ffi::ble_evt_user_mem_request_t,
    ) -> Self {
        Self {
            conn_handle,
            mem_type: FromPrimitive::from_u8((*e).type_).unwrap_or(BleMemType::Invalid),
        }
    }
}

impl BleEventDataType for CommonEventMemRequest {
    fn id() -> BleEventId {
        CommonEventId::MemRequest.into()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CommonEventMemRelease {
    pub mem_type: BleMemType,
    // TODO: Block Data
}

impl CommonEventMemRelease {
    pub(crate) unsafe fn from_c(e: *const ble_evt_user_mem_release_t) -> Self {
        Self {
            mem_type: FromPrimitive::from_u8((*e).type_).unwrap_or(BleMemType::Invalid),
        }
    }
}

impl BleEventDataType for CommonEventMemRelease {
    fn id() -> BleEventId {
        CommonEventId::MemRelease.into()
    }
}
