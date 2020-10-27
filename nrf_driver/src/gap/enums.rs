use crate::driver;


#[repr(u8)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BleGapAddressType {
    Public = driver::BLE_GAP_ADDR_TYPE_PUBLIC as u8,
    Static = driver::BLE_GAP_ADDR_TYPE_RANDOM_STATIC as u8,
    PrivateResolvable = driver::BLE_GAP_ADDR_TYPE_RANDOM_PRIVATE_RESOLVABLE as u8,
    PrivateNonresolvable = driver::BLE_GAP_ADDR_TYPE_RANDOM_PRIVATE_NON_RESOLVABLE as u8,
}


#[repr(u8)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BleGapAdvertisingType {
    ConnectableUndirected = driver::BLE_GAP_ADV_TYPE_ADV_IND as u8,
    ConnectableDirected = driver::BLE_GAP_ADV_TYPE_ADV_DIRECT_IND as u8,
    ScannableUndirected = driver::BLE_GAP_ADV_TYPE_ADV_SCAN_IND as u8,
    NonconnectableUndirected = driver::BLE_GAP_ADV_TYPE_ADV_NONCONN_IND as u8,
    ScanResponse = 0xFF
}


#[repr(u8)]
#[derive(FromPrimitive, Copy, Clone, Debug)]
pub enum BleGapTimeoutSource {
    Advertising = driver::BLE_GAP_TIMEOUT_SRC_ADVERTISING as u8,
    Scan = driver::BLE_GAP_TIMEOUT_SRC_SCAN as u8,
    Conn = driver::BLE_GAP_TIMEOUT_SRC_CONN as u8,
    AuthPayload = driver::BLE_GAP_TIMEOUT_SRC_AUTH_PAYLOAD as u8
}