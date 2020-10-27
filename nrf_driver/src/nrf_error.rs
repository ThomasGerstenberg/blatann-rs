use crate::driver;
use num_traits::FromPrimitive;
use std::fmt;

#[repr(u32)]
#[derive(FromPrimitive, Debug, Copy, Clone)]
pub enum NrfErrorType {
    Success = driver::NRF_SUCCESS,
    SvcHandlerMissing = driver::NRF_ERROR_SVC_HANDLER_MISSING,
    SoftdeviceNotEnabled = driver::NRF_ERROR_SOFTDEVICE_NOT_ENABLED,
    Internal = driver::NRF_ERROR_INTERNAL,
    NoMem = driver::NRF_ERROR_NO_MEM,
    NotFound = driver::NRF_ERROR_NOT_FOUND,
    NotSupported = driver::NRF_ERROR_NOT_SUPPORTED,
    InvalidParam = driver::NRF_ERROR_INVALID_PARAM,
    InvalidState = driver::NRF_ERROR_INVALID_STATE,
    InvalidLength = driver::NRF_ERROR_INVALID_LENGTH,
    InvalidFlags = driver::NRF_ERROR_INVALID_FLAGS,
    InvalidData = driver::NRF_ERROR_INVALID_DATA,
    DataSize = driver::NRF_ERROR_DATA_SIZE,
    Timeout = driver::NRF_ERROR_TIMEOUT,
    Null = driver::NRF_ERROR_NULL,
    Forbidden = driver::NRF_ERROR_FORBIDDEN,
    InvalidAddr = driver::NRF_ERROR_INVALID_ADDR,
    Busy = driver::NRF_ERROR_BUSY,
    ConnCount = driver::NRF_ERROR_CONN_COUNT,
    Resources = driver::NRF_ERROR_RESOURCES,
    BleNotEnabled = driver::BLE_ERROR_NOT_ENABLED,
    BleInvalidConnHandle = driver::BLE_ERROR_INVALID_CONN_HANDLE,
    BleInvalidAttrHandle = driver::BLE_ERROR_INVALID_ATTR_HANDLE,
    BleInvalidRole = driver::BLE_ERROR_INVALID_ROLE,
    SdRpcEncode = driver::NRF_ERROR_SD_RPC_ENCODE,
    SdRpcDecode = driver::NRF_ERROR_SD_RPC_DECODE,
    SdRpcSend = driver::NRF_ERROR_SD_RPC_SEND,
    SdRpcInvalidArgument = driver::NRF_ERROR_SD_RPC_INVALID_ARGUMENT,
    SdRpcNoResponse = driver::NRF_ERROR_SD_RPC_NO_RESPONSE,
    SdRpcInvalidState = driver::NRF_ERROR_SD_RPC_INVALID_STATE,
    SdRpcSerializationTransport = driver::NRF_ERROR_SD_RPC_SERIALIZATION_TRANSPORT,
    SdRpcSerializationTransportInvalidState = driver::NRF_ERROR_SD_RPC_SERIALIZATION_TRANSPORT_INVALID_STATE,
    SdRpcSerializationTransportNoResponse = driver::NRF_ERROR_SD_RPC_SERIALIZATION_TRANSPORT_NO_RESPONSE,
    SdRpcSerializationTransportAlreadyOpen = driver::NRF_ERROR_SD_RPC_SERIALIZATION_TRANSPORT_ALREADY_OPEN,
    SdRpcSerializationTransportAlreadyClosed = driver::NRF_ERROR_SD_RPC_SERIALIZATION_TRANSPORT_ALREADY_CLOSED,
    SdRpcH5Transport = driver::NRF_ERROR_SD_RPC_H5_TRANSPORT,
    SdRpcH5TransportState = driver::NRF_ERROR_SD_RPC_H5_TRANSPORT_STATE,
    SdRpcH5TransportNoResponse = driver::NRF_ERROR_SD_RPC_H5_TRANSPORT_NO_RESPONSE,
    SdRpcH5TransportSlipPayloadSize = driver::NRF_ERROR_SD_RPC_H5_TRANSPORT_SLIP_PAYLOAD_SIZE,
    SdRpcH5TransportSlipCalculatedPayloadSize = driver::NRF_ERROR_SD_RPC_H5_TRANSPORT_SLIP_CALCULATED_PAYLOAD_SIZE,
    SdRpcH5TransportSlipDecoding = driver::NRF_ERROR_SD_RPC_H5_TRANSPORT_SLIP_DECODING,
    SdRpcH5TransportHeaderChecksum = driver::NRF_ERROR_SD_RPC_H5_TRANSPORT_HEADER_CHECKSUM,
    SdRpcH5TransportPacketChecksum = driver::NRF_ERROR_SD_RPC_H5_TRANSPORT_PACKET_CHECKSUM,
    SdRpcH5TransportAlreadyOpen = driver::NRF_ERROR_SD_RPC_H5_TRANSPORT_ALREADY_OPEN,
    SdRpcH5TransportAlreadyClosed = driver::NRF_ERROR_SD_RPC_H5_TRANSPORT_ALREADY_CLOSED,
    SdRpcH5TransportInternalError = driver::NRF_ERROR_SD_RPC_H5_TRANSPORT_INTERNAL_ERROR,
    SdRpcSerialPort = driver::NRF_ERROR_SD_RPC_SERIAL_PORT,
    SdRpcSerialPortState = driver::NRF_ERROR_SD_RPC_SERIAL_PORT_STATE,
    SdRpcSerialPortAlreadyOpen = driver::NRF_ERROR_SD_RPC_SERIAL_PORT_ALREADY_OPEN,
    SdRpcSerialPortAlreadyClosed = driver::NRF_ERROR_SD_RPC_SERIAL_PORT_ALREADY_CLOSED,
    SdRpcSerialPortInternalError = driver::NRF_ERROR_SD_RPC_SERIAL_PORT_INTERNAL_ERROR,
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
        if err == driver::NRF_SUCCESS {
            Ok(())
        }
        else {
            Err(NrfError::new(err))
        }
    }

    pub fn make_result_typed<T: Sized, F>(err: u32, f: F) -> Result<T, NrfError>
        where F: FnOnce() -> T {
        if err == driver::NRF_SUCCESS {
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