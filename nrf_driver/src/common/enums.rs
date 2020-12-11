use crate::ffi;

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
    RemoteDevTerminationDueToLowResources = ffi::BLE_HCI_REMOTE_DEV_TERMINATION_DUE_TO_LOW_RESOURCES as u8,
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
