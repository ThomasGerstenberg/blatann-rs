use crate::ffi;
use crate::common::types::ConnHandle;

pub const CONN_HANDLE_INVALID: ConnHandle = ffi::BLE_CONN_HANDLE_INVALID as ConnHandle;
