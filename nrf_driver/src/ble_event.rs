use num_traits::{FromPrimitive, ToPrimitive};

use crate::common::events::*;
use crate::ffi::{ble_common_evt_t, ble_evt_t};
use crate::ffi;
use crate::gap::events::*;

#[derive(Copy, Clone, Debug)]
pub enum BleEventId {
    Common(CommonEventId),
    Gap(GapEventId),
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
            BleEventId::Common(x) => x as u16,
            BleEventId::Gap(x) => x as u16
        }
    }
}


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

impl Into<BleEventId> for CommonEventId {
    fn into(self) -> BleEventId {
        BleEventId::Common(self)
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

impl Into<BleEventId> for GapEventId {
    fn into(self) -> BleEventId {
        BleEventId::Gap(self)
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
pub struct BleEvent {
    pub id: u16,
    pub data: Option<BleEventData>,
}

impl BleEvent {
    pub unsafe fn from_c(e: *const ble_evt_t) -> Self {
        Self {
            id: (*e).header.evt_id,
            data: BleEventData::from_c(e),
        }
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

pub trait BleEventDataType: Clone {
    fn id() -> BleEventId;
}
