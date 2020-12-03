use std::convert::TryInto;

use num_traits::FromPrimitive;

use crate::ffi;
use crate::utils::*;

use super::enums::*;

const ADDR_LEN: usize = 6;


#[derive(Debug, Copy, Clone)]
pub struct BleGapAddress {
    pub address_type: BleGapAddressType,
    pub address: [u8; ADDR_LEN],
}


impl BleGapAddress {
    pub fn from_c(addr: *const ffi::ble_gap_addr_t) -> Self {
        unsafe {
            let addr = addr.as_ref().unwrap();
            let mut addr_data = addr.addr;
            addr_data.reverse();
            let addr_type = FromPrimitive::from_u8(addr.addr_type()).unwrap();
            Self {
                address_type: addr_type,
                address: addr_data,
            }
        }
    }

    pub fn new(addr_str: String, addr_type: BleGapAddressType) -> Self {
        let addr = addr_str
            .split(":")
            .map(|s| u8::from_str_radix(s, 16).unwrap())
            .collect::<Vec<u8>>()
            .as_slice()
            .try_into()
            .unwrap();

        Self {
            address_type: addr_type,
            address: addr,
        }
    }

    pub fn to_string(&self) -> String {
        let parts = self.address.iter().map(|x| format! {"{:02X}", x}).collect::<Vec<String>>();
        return parts.join(":");
    }

    pub fn to_c(&self) -> ffi::ble_gap_addr_t {
        let mut addr = self.address.clone();
        addr.reverse();

        ffi::ble_gap_addr_t {
            _bitfield_1: ffi::ble_gap_addr_t::new_bitfield_1(0, self.address_type as u8),
            addr,
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct BleGapAdvParams {
    pub interval: Milliseconds,
    pub timeout_s: u16,
    pub advertising_type: BleGapAdvertisingType,
}


impl BleGapAdvParams {
    pub fn new(interval: Milliseconds, timeout_s: u16, advertising_type: BleGapAdvertisingType) -> Self {
        Self {
            interval,
            timeout_s,
            advertising_type,
        }
    }

    pub fn default() -> Self {
        Self::new(40f64, 180, BleGapAdvertisingType::ConnectableUndirected)
    }

    pub fn to_c(&self) -> ffi::ble_gap_adv_params_t {
        ffi::ble_gap_adv_params_t {
            type_: self.advertising_type as u8,
            p_peer_addr: std::ptr::null(),
            fp: ffi::BLE_GAP_ADV_FP_ANY as u8,
            interval: self.interval.to_units(UNIT_0_625_MS) as u16,
            timeout: self.timeout_s,
            channel_mask: ffi::ble_gap_adv_ch_mask_t {
                _bitfield_1: ffi::ble_gap_adv_ch_mask_t::new_bitfield_1(0, 0, 0)
            },
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct BleGapConnParams {
    pub min_interval: Milliseconds,
    pub max_interval: Milliseconds,
    pub timeout: Milliseconds,
    pub slave_latency: u16
}

impl BleGapConnParams {
    pub fn new(min_interval: Milliseconds, max_interval: Milliseconds,
               timeout: Milliseconds, slave_latency: u16) -> Self {
        Self {
            min_interval,
            max_interval,
            timeout,
            slave_latency,
        }
    }

    pub fn from_c(conn_params: *const ffi::ble_gap_conn_params_t) -> Self {
        let (min_interval, max_interval, timeout, slave_latency) = unsafe {
            ((*conn_params).min_conn_interval,
             (*conn_params).max_conn_interval,
             (*conn_params).conn_sup_timeout,
             (*conn_params).slave_latency)
        };
        Self {
            min_interval: (min_interval as Units).to_ms(UNIT_1_25_MS),
            max_interval: (max_interval as Units).to_ms(UNIT_1_25_MS),
            timeout: (timeout as Units).to_ms(UNIT_10_MS),
            slave_latency,
        }
    }

    pub fn to_c(&self) -> ffi::ble_gap_conn_params_t {
        ffi::ble_gap_conn_params_t {
            min_conn_interval: self.min_interval.to_units(UNIT_1_25_MS) as u16,
            max_conn_interval: self.max_interval.to_units(UNIT_1_25_MS) as u16,
            conn_sup_timeout: self.timeout.to_units(UNIT_10_MS) as u16,
            slave_latency: self.slave_latency,
        }
    }
}