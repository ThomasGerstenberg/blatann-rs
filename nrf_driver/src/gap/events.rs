use num_traits::FromPrimitive;

use crate::common::types::ConnHandle;
use crate::ffi;

use super::enums::*;
use super::types::*;
use crate::common::enums::BleHciStatus;

#[repr(u16)]
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug)]
pub enum GapEventId {
    Connected = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_CONNECTED as u16,
    Disconnected = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_DISCONNECTED as u16,
    // ConnParamUpdate = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_CONN_PARAM_UPDATE as u16,
    // SecParamsRequest = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_SEC_PARAMS_REQUEST as u16,
    // SecInfoRequest = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_SEC_INFO_REQUEST as u16,
    // PasskeyDisplay = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_PASSKEY_DISPLAY as u16,
    // KeyPressed = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_KEY_PRESSED as u16,
    // AuthKeyRequest = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_AUTH_KEY_REQUEST as u16,
    // LescDhkeyRequest = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_LESC_DHKEY_REQUEST as u16,
    // AuthStatus = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_AUTH_STATUS as u16,
    // ConnSecUpdate = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_CONN_SEC_UPDATE as u16,
    Timeout = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_TIMEOUT as u16,
    // RssiChanged = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_RSSI_CHANGED as u16,
    // AdvReport = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_ADV_REPORT as u16,
    // SecRequest = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_SEC_REQUEST as u16,
    // ConnParamUpdateRequest = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_CONN_PARAM_UPDATE_REQUEST as u16,
    // ScanReqReport = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_SCAN_REQ_REPORT as u16,
    PhyUpdateRequest = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_PHY_UPDATE_REQUEST as u16,
    PhyUpdate = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_PHY_UPDATE as u16,
    DataLengthUpdateRequest = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_DATA_LENGTH_UPDATE_REQUEST as u16,
    DataLengthUpdate = ffi::BLE_GAP_EVTS_BLE_GAP_EVT_DATA_LENGTH_UPDATE as u16,
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
    PhyUpdateRequest(GapEventPhyUpdateRequest),
    PhyUpdate(GapEventPhyUpdate),
    DataLengthUpdateRequest(GapEventDataLengthUpdateRequest),
    DataLengthUpdate(GapEventDataLengthUpdate),
}

impl GapEvent {
    pub(crate) unsafe fn from_c(id: GapEventId, e: *const ffi::ble_gap_evt_t) -> Self {
        let conn_handle = (*e).conn_handle;
        let params = &(*e).params;
        match id {
            GapEventId::Connected => GapEvent::Connected(GapEventConnected::from_c(conn_handle, &params.connected)),
            GapEventId::Disconnected => GapEvent::Disconnected(GapEventDisconnected::from_c(conn_handle, &params.disconnected)),
            // GapEventId::ConnParamUpdate => unimplemented!(),
            // GapEventId::SecParamsRequest => unimplemented!(),
            // GapEventId::SecInfoRequest => unimplemented!(),
            // GapEventId::PasskeyDisplay => unimplemented!(),
            // GapEventId::KeyPressed => unimplemented!(),
            // GapEventId::AuthKeyRequest => unimplemented!(),
            // GapEventId::LescDhkeyRequest => unimplemented!(),
            // GapEventId::AuthStatus => unimplemented!(),
            // GapEventId::ConnSecUpdate => unimplemented!(),
            GapEventId::Timeout => GapEvent::Timeout(GapEventTimeout::from_c(conn_handle, &params.timeout)),
            // GapEventId::RssiChanged => unimplemented!(),
            // GapEventId::AdvReport => unimplemented!(),
            // GapEventId::SecRequest => unimplemented!(),
            // GapEventId::ConnParamUpdateRequest => unimplemented!(),
            // GapEventId::ScanReqReport => unimplemented!(),
            GapEventId::PhyUpdateRequest => GapEvent::PhyUpdateRequest(GapEventPhyUpdateRequest::from_c(conn_handle, &params.phy_update_request)),
            GapEventId::PhyUpdate => GapEvent::PhyUpdate(GapEventPhyUpdate::from_c(conn_handle, &params.phy_update)),
            GapEventId::DataLengthUpdateRequest => GapEvent::DataLengthUpdateRequest(GapEventDataLengthUpdateRequest::from_c(conn_handle, &params.data_length_update_request)),
            GapEventId::DataLengthUpdate => GapEvent::DataLengthUpdate(GapEventDataLengthUpdate::from_c(conn_handle, &params.data_length_update)),
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
            address: (*val).peer_addr.into(),
            role: FromPrimitive::from_u8((*val).role).unwrap_or(BleGapRole::Invalid),
            conn_params: (*val).conn_params.into()
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


#[derive(Debug, Copy, Clone)]
pub struct GapEventPhyUpdateRequest {
    pub conn_handle: ConnHandle,
    pub peer_preferred_phys: BleGapPhys,
}

impl GapEventPhyUpdateRequest {
    pub fn id() -> u16 {
        GapEventId::PhyUpdateRequest as u16
    }

    unsafe fn from_c(conn_handle: ConnHandle, val: *const ffi::ble_gap_evt_phy_update_request_t) -> Self {
        Self {
            conn_handle,
            peer_preferred_phys: (*val).peer_preferred_phys.into() //BleGapPhys::from(&(*val).peer_preferred_phys)
        }
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
    pub fn id() -> u16 {
        GapEventId::PhyUpdate as u16
    }

    unsafe fn from_c(conn_handle: ConnHandle, val: *const ffi::ble_gap_evt_phy_update_t) -> Self {
        Self {
            conn_handle,
            status: FromPrimitive::from_u8((*val).status).unwrap_or_else(|| BleHciStatus::InvalidHciCode),
            tx_phy: BleGapPhy::from_bits_or_default((*val).tx_phy),
            rx_phy: BleGapPhy::from_bits_or_default((*val).rx_phy),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GapEventDataLengthUpdateRequest {
    pub conn_handle: ConnHandle,
    pub peer_params: BleGapDataLengthParams,
}

impl GapEventDataLengthUpdateRequest {
    pub fn id() -> u16 {
        GapEventId::DataLengthUpdateRequest as u16
    }

    unsafe fn from_c(conn_handle: ConnHandle, val: *const ffi::ble_gap_evt_data_length_update_request_t) -> Self {
        Self {
            conn_handle,
            peer_params: (*val).peer_params.into(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GapEventDataLengthUpdate {
    pub conn_handle: ConnHandle,
    pub effective_params: BleGapDataLengthParams,
}

impl GapEventDataLengthUpdate {
    pub fn id() -> u16 {
        GapEventId::DataLengthUpdate as u16
    }

    unsafe fn from_c(conn_handle: ConnHandle, val: *const ffi::ble_gap_evt_data_length_update_t) -> Self {
        Self {
            conn_handle,
            effective_params: (*val).effective_params.into(),
        }
    }
}
