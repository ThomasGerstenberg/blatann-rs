use num_traits::FromPrimitive;

use crate::common::types::ConnHandle;
use crate::ffi;

use super::enums::*;
use super::types::*;
use crate::ble_event::{BleEventDataType, BleEventId, GapEventId};
use crate::common::enums::BleHciStatus;

#[derive(Copy, Clone, Debug)]
pub struct GapEventConnected {
    pub conn_handle: ConnHandle,
    pub address: BleGapAddress,
    pub role: BleGapRole,
    pub conn_params: BleGapConnParams,
}

impl GapEventConnected {
    pub fn id() -> u16 {
        GapEventId::Connected as u16
    }

    pub(crate) unsafe fn from_c(
        conn_handle: ConnHandle,
        val: *const ffi::ble_gap_evt_connected_t,
    ) -> Self {
        Self {
            conn_handle,
            address: (*val).peer_addr.into(),
            role: FromPrimitive::from_u8((*val).role).unwrap_or(BleGapRole::Invalid),
            conn_params: (*val).conn_params.into(),
        }
    }
}

impl BleEventDataType for GapEventConnected {
    fn id() -> BleEventId {
        GapEventId::Connected.into()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GapEventDisconnected {
    pub conn_handle: ConnHandle,
    pub reason: BleHciStatus,
}

impl GapEventDisconnected {
    pub(crate) unsafe fn from_c(
        conn_handle: ConnHandle,
        val: *const ffi::ble_gap_evt_disconnected_t,
    ) -> Self {
        Self {
            conn_handle,
            reason: FromPrimitive::from_u8((*val).reason).unwrap_or(BleHciStatus::InvalidHciCode),
        }
    }
}

impl BleEventDataType for GapEventDisconnected {
    fn id() -> BleEventId {
        GapEventId::Disconnected.into()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GapEventTimeout {
    pub conn_handle: ConnHandle,
    pub src: BleGapTimeoutSource,
}

impl GapEventTimeout {
    pub(crate) unsafe fn from_c(
        conn_handle: ConnHandle,
        val: *const ffi::ble_gap_evt_timeout_t,
    ) -> Self {
        Self {
            conn_handle,
            src: FromPrimitive::from_u8((*val).src).unwrap(),
        }
    }
}

impl BleEventDataType for GapEventTimeout {
    fn id() -> BleEventId {
        GapEventId::Timeout.into()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GapEventPhyUpdateRequest {
    pub conn_handle: ConnHandle,
    pub peer_preferred_phys: BleGapPhys,
}

impl GapEventPhyUpdateRequest {
    pub(crate) unsafe fn from_c(
        conn_handle: ConnHandle,
        val: *const ffi::ble_gap_evt_phy_update_request_t,
    ) -> Self {
        Self {
            conn_handle,
            peer_preferred_phys: (*val).peer_preferred_phys.into(), //BleGapPhys::from(&(*val).peer_preferred_phys)
        }
    }
}

impl BleEventDataType for GapEventPhyUpdateRequest {
    fn id() -> BleEventId {
        GapEventId::PhyUpdateRequest.into()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GapEventPhyUpdate {
    pub conn_handle: ConnHandle,
    pub status: BleHciStatus,
    pub tx_phy: BleGapPhy,
    pub rx_phy: BleGapPhy,
}

impl GapEventPhyUpdate {
    pub(crate) unsafe fn from_c(
        conn_handle: ConnHandle,
        val: *const ffi::ble_gap_evt_phy_update_t,
    ) -> Self {
        Self {
            conn_handle,
            status: FromPrimitive::from_u8((*val).status)
                .unwrap_or_else(|| BleHciStatus::InvalidHciCode),
            tx_phy: BleGapPhy::from_bits_or_default((*val).tx_phy),
            rx_phy: BleGapPhy::from_bits_or_default((*val).rx_phy),
        }
    }
}

impl BleEventDataType for GapEventPhyUpdate {
    fn id() -> BleEventId {
        GapEventId::PhyUpdate.into()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GapEventDataLengthUpdateRequest {
    pub conn_handle: ConnHandle,
    pub peer_params: BleGapDataLengthParams,
}

impl GapEventDataLengthUpdateRequest {
    pub(crate) unsafe fn from_c(
        conn_handle: ConnHandle,
        val: *const ffi::ble_gap_evt_data_length_update_request_t,
    ) -> Self {
        Self {
            conn_handle,
            peer_params: (*val).peer_params.into(),
        }
    }
}

impl BleEventDataType for GapEventDataLengthUpdateRequest {
    fn id() -> BleEventId {
        GapEventId::DataLengthUpdateRequest.into()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GapEventDataLengthUpdate {
    pub conn_handle: ConnHandle,
    pub effective_params: BleGapDataLengthParams,
}

impl GapEventDataLengthUpdate {
    pub(crate) unsafe fn from_c(
        conn_handle: ConnHandle,
        val: *const ffi::ble_gap_evt_data_length_update_t,
    ) -> Self {
        Self {
            conn_handle,
            effective_params: (*val).effective_params.into(),
        }
    }
}

impl BleEventDataType for GapEventDataLengthUpdate {
    fn id() -> BleEventId {
        GapEventId::DataLengthUpdate.into()
    }
}
