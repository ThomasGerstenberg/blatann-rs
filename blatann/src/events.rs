use nrf_driver::common::enums::BleHciStatus;
use crate::peer::Phy;

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

#[derive(Debug, Copy, Clone)]
pub struct PhyUpdateEvent {
    pub tx_phy: Phy,
    pub rx_phy: Phy,
}

#[derive(Debug, Copy, Clone)]
pub struct DataLengthUpdateEvent {
    pub tx_bytes: u16,
    pub rx_bytes: u16,
    pub tx_time_us: u16,
    pub rx_time_us: u16,
}