use std::collections::HashMap;
use nrf_driver::gap::enums::{BleAdvDataType};

pub const MAX_ADVERTISE_ENCODED_LEN: usize = 31;

bitflags! {
    pub struct AdvertisingFlags: u8 {
        const LIMITED_DISCOVERY_MODE = 0x01;
        const GENERAL_DISCOVERY_MODE = 0x02;
        const BR_EDR_NOT_SUPPORTED = 0x04;
        const BR_EDR_CONTROLLER = 0x08;
        const BR_EDR_HOST = 0x10;
    }
}

pub type AdvDataType = BleAdvDataType;

// TODO: Rest of API
pub struct AdvData {
    pub entries: HashMap<u8, Vec<u8>>,
}

impl Default for AdvData {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl AdvData {
    pub fn add_entry(&mut self, adv_type: u8, data: &[u8]) {
        self.entries.insert(adv_type, data.to_vec());
    }

    pub fn serialize(&self) -> Vec<u8> {
        // Data is in length-type-value format
        let mut adv_data = Vec::new();
        for (adv_type, data) in self.entries.iter() {
            adv_data.push((data.len() + 1) as u8);
            adv_data.push(*adv_type);
            adv_data.extend(data);
        };

        return adv_data;
    }
}
