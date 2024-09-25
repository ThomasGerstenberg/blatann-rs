use crate::ffi;
use log::Level;

#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug)]
pub enum BleHciStatus {
    Success = ffi::BLE_HCI_STATUS_CODE_SUCCESS as u8,
    UnknownBtleCommand = ffi::BLE_HCI_STATUS_CODE_UNKNOWN_BTLE_COMMAND as u8,
    UnknownConnectionIdentifier = ffi::BLE_HCI_STATUS_CODE_UNKNOWN_CONNECTION_IDENTIFIER as u8,
    AuthenticationFailure = ffi::BLE_HCI_AUTHENTICATION_FAILURE as u8,
    PinOrKeyMissing = ffi::BLE_HCI_STATUS_CODE_PIN_OR_KEY_MISSING as u8,
    MemoryCapacityExceeded = ffi::BLE_HCI_MEMORY_CAPACITY_EXCEEDED as u8,
    ConnectionTimeout = ffi::BLE_HCI_CONNECTION_TIMEOUT as u8,
    CommandDisallowed = ffi::BLE_HCI_STATUS_CODE_COMMAND_DISALLOWED as u8,
    InvalidBtleCommandParameters = ffi::BLE_HCI_STATUS_CODE_INVALID_BTLE_COMMAND_PARAMETERS as u8,
    RemoteUserTerminatedConnection = ffi::BLE_HCI_REMOTE_USER_TERMINATED_CONNECTION as u8,
    RemoteDevTerminationDueToLowResources =
        ffi::BLE_HCI_REMOTE_DEV_TERMINATION_DUE_TO_LOW_RESOURCES as u8,
    RemoteDevTerminationDueToPowerOff = ffi::BLE_HCI_REMOTE_DEV_TERMINATION_DUE_TO_POWER_OFF as u8,
    LocalHostTerminatedConnection = ffi::BLE_HCI_LOCAL_HOST_TERMINATED_CONNECTION as u8,
    UnsupportedRemoteFeature = ffi::BLE_HCI_UNSUPPORTED_REMOTE_FEATURE as u8,
    InvalidLmpParameters = ffi::BLE_HCI_STATUS_CODE_INVALID_LMP_PARAMETERS as u8,
    UnspecifiedError = ffi::BLE_HCI_STATUS_CODE_UNSPECIFIED_ERROR as u8,
    LmpResponseTimeout = ffi::BLE_HCI_STATUS_CODE_LMP_RESPONSE_TIMEOUT as u8,
    LmpErrorTransactionCollision = ffi::BLE_HCI_STATUS_CODE_LMP_ERROR_TRANSACTION_COLLISION as u8,
    LmpPduNotAllowed = ffi::BLE_HCI_STATUS_CODE_LMP_PDU_NOT_ALLOWED as u8,
    InstantPassed = ffi::BLE_HCI_INSTANT_PASSED as u8,
    PairingWithUnitKeyUnsupported = ffi::BLE_HCI_PAIRING_WITH_UNIT_KEY_UNSUPPORTED as u8,
    DifferentTransactionCollision = ffi::BLE_HCI_DIFFERENT_TRANSACTION_COLLISION as u8,
    ParameterOutOfMandatoryRange = ffi::BLE_HCI_PARAMETER_OUT_OF_MANDATORY_RANGE as u8,
    ControllerBusy = ffi::BLE_HCI_CONTROLLER_BUSY as u8,
    ConnIntervalUnacceptable = ffi::BLE_HCI_CONN_INTERVAL_UNACCEPTABLE as u8,
    DirectedAdvertiserTimeout = ffi::BLE_HCI_DIRECTED_ADVERTISER_TIMEOUT as u8,
    ConnTerminatedDueToMicFailure = ffi::BLE_HCI_CONN_TERMINATED_DUE_TO_MIC_FAILURE as u8,
    ConnFailedToBeEstablished = ffi::BLE_HCI_CONN_FAILED_TO_BE_ESTABLISHED as u8,
    InvalidHciCode = 0xFF,
}

#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug)]
pub enum BleMemType {
    Invalid = ffi::BLE_USER_MEM_TYPE_INVALID as u8,
    GattsQueuedWrites = ffi::BLE_USER_MEM_TYPE_GATTS_QUEUED_WRITES as u8,
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug)]
pub enum BleLogSeverity {
    Trace = ffi::sd_rpc_log_severity_t_SD_RPC_LOG_TRACE as u32,
    Debug = ffi::sd_rpc_log_severity_t_SD_RPC_LOG_DEBUG as u32,
    Info = ffi::sd_rpc_log_severity_t_SD_RPC_LOG_INFO as u32,
    Warning = ffi::sd_rpc_log_severity_t_SD_RPC_LOG_WARNING as u32,
    Error = ffi::sd_rpc_log_severity_t_SD_RPC_LOG_ERROR as u32,
    Fatal = ffi::sd_rpc_log_severity_t_SD_RPC_LOG_FATAL as u32,
}

impl Into<Level> for BleLogSeverity {
    fn into(self) -> Level {
        match self {
            BleLogSeverity::Trace => Level::Trace,
            BleLogSeverity::Debug => Level::Debug,
            BleLogSeverity::Info => Level::Info,
            BleLogSeverity::Warning => Level::Warn,
            BleLogSeverity::Error => Level::Error,
            BleLogSeverity::Fatal => Level::Error,
        }
    }
}

#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug)]
pub enum RpcAppStatus {
    PacketSendMaxTriesReached = ffi::sd_rpc_app_status_t_PKT_SEND_MAX_RETRIES_REACHED as u32,
    PacketUnexpected = ffi::sd_rpc_app_status_t_PKT_UNEXPECTED as u32,
    PacketEncodeError = ffi::sd_rpc_app_status_t_PKT_ENCODE_ERROR as u32,
    PacketDecodeError = ffi::sd_rpc_app_status_t_PKT_DECODE_ERROR as u32,
    PacketSendError = ffi::sd_rpc_app_status_t_PKT_SEND_ERROR as u32,
    IoResourcesUnavailable = ffi::sd_rpc_app_status_t_IO_RESOURCES_UNAVAILABLE as u32,
    ResetPerformed = ffi::sd_rpc_app_status_t_RESET_PERFORMED as u32,
    ConnectionActive = ffi::sd_rpc_app_status_t_CONNECTION_ACTIVE as u32,
    Unknown = 8u32,
}
