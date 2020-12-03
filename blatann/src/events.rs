use nrf_driver::common::enums::BleHciStatus;

// No params
#[derive(Debug, Copy, Clone)]
pub struct AdvertisingTimeoutEvent {}

// No params (yet)
#[derive(Debug, Copy, Clone)]
pub struct ConnectionEvent {}

#[derive(Debug, Copy, Clone)]
pub struct DisconnectionEvent {
    pub reason: BleHciStatus,
}