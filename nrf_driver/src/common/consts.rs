use crate::common::types::ConnHandle;
use crate::ffi;

pub const CONN_HANDLE_INVALID: ConnHandle = ffi::BLE_CONN_HANDLE_INVALID as ConnHandle;
