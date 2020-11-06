use crate::ffi;
use num_traits::FromPrimitive;
use std::fmt;

#[repr(u32)]
#[derive(FromPrimitive, Debug, Copy, Clone)]
pub enum NrfErrorType {
    Success = ffi::NRF_SUCCESS,
    SvcHandlerMissing = ffi::NRF_ERROR_SVC_HANDLER_MISSING,
    SoftdeviceNotEnabled = ffi::NRF_ERROR_SOFTDEVICE_NOT_ENABLED,
    Internal = ffi::NRF_ERROR_INTERNAL,
    NoMem = ffi::NRF_ERROR_NO_MEM,
    NotFound = ffi::NRF_ERROR_NOT_FOUND,
    NotSupported = ffi::NRF_ERROR_NOT_SUPPORTED,
    InvalidParam = ffi::NRF_ERROR_INVALID_PARAM,
    InvalidState = ffi::NRF_ERROR_INVALID_STATE,
    InvalidLength = ffi::NRF_ERROR_INVALID_LENGTH,
    InvalidFlags = ffi::NRF_ERROR_INVALID_FLAGS,
    InvalidData = ffi::NRF_ERROR_INVALID_DATA,
    DataSize = ffi::NRF_ERROR_DATA_SIZE,
    Timeout = ffi::NRF_ERROR_TIMEOUT,
    Null = ffi::NRF_ERROR_NULL,
    Forbidden = ffi::NRF_ERROR_FORBIDDEN,
    InvalidAddr = ffi::NRF_ERROR_INVALID_ADDR,
    Busy = ffi::NRF_ERROR_BUSY,
    ConnCount = ffi::NRF_ERROR_CONN_COUNT,
    Resources = ffi::NRF_ERROR_RESOURCES,
    BleNotEnabled = ffi::BLE_ERROR_NOT_ENABLED,
    BleInvalidConnHandle = ffi::BLE_ERROR_INVALID_CONN_HANDLE,
    BleInvalidAttrHandle = ffi::BLE_ERROR_INVALID_ATTR_HANDLE,
    BleInvalidRole = ffi::BLE_ERROR_INVALID_ROLE,
    SdRpcEncode = ffi::NRF_ERROR_SD_RPC_ENCODE,
    SdRpcDecode = ffi::NRF_ERROR_SD_RPC_DECODE,
    SdRpcSend = ffi::NRF_ERROR_SD_RPC_SEND,
    SdRpcInvalidArgument = ffi::NRF_ERROR_SD_RPC_INVALID_ARGUMENT,
    SdRpcNoResponse = ffi::NRF_ERROR_SD_RPC_NO_RESPONSE,
    SdRpcInvalidState = ffi::NRF_ERROR_SD_RPC_INVALID_STATE,
    SdRpcSerializationTransport = ffi::NRF_ERROR_SD_RPC_SERIALIZATION_TRANSPORT,
    SdRpcSerializationTransportInvalidState = ffi::NRF_ERROR_SD_RPC_SERIALIZATION_TRANSPORT_INVALID_STATE,
    SdRpcSerializationTransportNoResponse = ffi::NRF_ERROR_SD_RPC_SERIALIZATION_TRANSPORT_NO_RESPONSE,
    SdRpcSerializationTransportAlreadyOpen = ffi::NRF_ERROR_SD_RPC_SERIALIZATION_TRANSPORT_ALREADY_OPEN,
    SdRpcSerializationTransportAlreadyClosed = ffi::NRF_ERROR_SD_RPC_SERIALIZATION_TRANSPORT_ALREADY_CLOSED,
    SdRpcH5Transport = ffi::NRF_ERROR_SD_RPC_H5_TRANSPORT,
    SdRpcH5TransportState = ffi::NRF_ERROR_SD_RPC_H5_TRANSPORT_STATE,
    SdRpcH5TransportNoResponse = ffi::NRF_ERROR_SD_RPC_H5_TRANSPORT_NO_RESPONSE,
    SdRpcH5TransportSlipPayloadSize = ffi::NRF_ERROR_SD_RPC_H5_TRANSPORT_SLIP_PAYLOAD_SIZE,
    SdRpcH5TransportSlipCalculatedPayloadSize = ffi::NRF_ERROR_SD_RPC_H5_TRANSPORT_SLIP_CALCULATED_PAYLOAD_SIZE,
    SdRpcH5TransportSlipDecoding = ffi::NRF_ERROR_SD_RPC_H5_TRANSPORT_SLIP_DECODING,
    SdRpcH5TransportHeaderChecksum = ffi::NRF_ERROR_SD_RPC_H5_TRANSPORT_HEADER_CHECKSUM,
    SdRpcH5TransportPacketChecksum = ffi::NRF_ERROR_SD_RPC_H5_TRANSPORT_PACKET_CHECKSUM,
    SdRpcH5TransportAlreadyOpen = ffi::NRF_ERROR_SD_RPC_H5_TRANSPORT_ALREADY_OPEN,
    SdRpcH5TransportAlreadyClosed = ffi::NRF_ERROR_SD_RPC_H5_TRANSPORT_ALREADY_CLOSED,
    SdRpcH5TransportInternalError = ffi::NRF_ERROR_SD_RPC_H5_TRANSPORT_INTERNAL_ERROR,
    SdRpcSerialPort = ffi::NRF_ERROR_SD_RPC_SERIAL_PORT,
    SdRpcSerialPortState = ffi::NRF_ERROR_SD_RPC_SERIAL_PORT_STATE,
    SdRpcSerialPortAlreadyOpen = ffi::NRF_ERROR_SD_RPC_SERIAL_PORT_ALREADY_OPEN,
    SdRpcSerialPortAlreadyClosed = ffi::NRF_ERROR_SD_RPC_SERIAL_PORT_ALREADY_CLOSED,
    SdRpcSerialPortInternalError = ffi::NRF_ERROR_SD_RPC_SERIAL_PORT_INTERNAL_ERROR,
    Unknown = 0xFFFFFFFF,
}

impl NrfErrorType {
    pub fn from(err: u32) -> Self {
        match FromPrimitive::from_u32(err) {
            Some(x) => x,
            None => NrfErrorType::Unknown
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct NrfError {
    pub error_type: NrfErrorType,
    pub error_code: u32,
}

impl NrfError {
    pub fn new(err: u32) -> Self {
        Self {
            error_type: NrfErrorType::from(err),
            error_code: err
        }
    }

    pub fn make_result(err: u32) -> Result<(), Self> {
        if err == ffi::NRF_SUCCESS {
            Ok(())
        }
        else {
            Err(NrfError::new(err))
        }
    }

    pub fn make_result_typed<T: Sized, F>(err: u32, f: F) -> Result<T, NrfError>
        where F: FnOnce() -> T {
        if err == ffi::NRF_SUCCESS {
            Ok(f())
        }
        else {
            Err(NrfError::new(err))
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{:?}({})", self.error_type, self.error_code)
    }
}

impl fmt::Display for NrfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}