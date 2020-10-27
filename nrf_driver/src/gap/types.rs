use std::convert::TryInto;
use num_traits::FromPrimitive;

use crate::driver;
use crate::utils::*;
use super::enums::*;

const ADDR_LEN: usize = 6;


#[derive(Debug, Copy, Clone)]
pub struct BleGapAddress {
    pub address_type: BleGapAddressType,
    pub address: [u8; ADDR_LEN]
}


impl BleGapAddress {
    pub fn new_from_c(addr: *const driver::ble_gap_addr_t) -> Self {
        unsafe {
            let addr = addr.as_ref().unwrap();
            let mut addr_data = addr.addr;
            addr_data.reverse();
            let addr_type = FromPrimitive::from_u8(addr.addr_type()).unwrap();
            Self {
                address_type: addr_type,
                address: addr_data
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
            address: addr
        }
    }

    pub fn to_string(&self) -> String {
        let parts= self.address.iter().map(|x| format!{"{:02X}", x}).collect::<Vec<String>>();
        return parts.join(":");
    }

    pub fn to_c(&self) -> driver::ble_gap_addr_t {
       let mut addr = self.address.clone();
        addr.reverse();

        driver::ble_gap_addr_t {
            _bitfield_1: driver::ble_gap_addr_t::new_bitfield_1(0, self.address_type as u8),
            addr
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct BleGapAdvParams {
    pub interval: Milliseconds,
    pub timeout_s: u16,
    pub advertising_type: BleGapAdvertisingType
}


impl BleGapAdvParams {
    pub fn new(interval: Milliseconds, timeout_s: u16, advertising_type: BleGapAdvertisingType) -> Self {
        Self {
            interval,
            timeout_s,
            advertising_type
        }
    }

    pub fn default() -> Self {
        Self::new(40f64, 180, BleGapAdvertisingType::ConnectableUndirected)
    }

    pub fn to_c(&self) -> driver::ble_gap_adv_params_t {
        driver::ble_gap_adv_params_t {
            type_: self.advertising_type as u8,
            p_peer_addr: std::ptr::null(),
            fp: driver::BLE_GAP_ADV_FP_ANY as u8,
            interval: self.interval.to_units(UNIT_0_625_MS) as u16,
            timeout: self.timeout_s,
            channel_mask: driver::ble_gap_adv_ch_mask_t {
                _bitfield_1: driver::ble_gap_adv_ch_mask_t::new_bitfield_1(0, 0, 0)
            }
        }
    }
}
