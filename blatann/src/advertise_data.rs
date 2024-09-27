use nrf_driver::gap::enums::BleAdvDataType;
use std::collections::HashMap;

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

impl From<AdvertisingFlags> for u8 {
    fn from(value: AdvertisingFlags) -> Self {
        value.bits()
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

    pub fn set_flags(&mut self, flags: AdvertisingFlags) {
        self.entries
            .insert(AdvDataType::Flags.into(), vec![flags.bits()]);
    }

    pub fn set_name(&mut self, name: &str, is_complete: bool) {
        let adv_type = if is_complete {
            AdvDataType::CompleteLocalName
        } else {
            AdvDataType::ShortLocalName
        };
        let data = name.as_bytes();
        self.add_entry(adv_type.into(), data.into());
    }

    pub fn set_service_uuid16s(&mut self, uuids: &[u16], is_complete_list: bool) {
        let adv_type = if is_complete_list {
            AdvDataType::Service16bitUuidComplete
        } else {
            AdvDataType::Service16bitUuidMoreAvailable
        };
        let data: Vec<u8> = uuids
            .iter()
            .map(|x| [(x & 0xFF) as u8, (x >> 8) as u8])
            .flatten()
            .collect();
        self.add_entry(adv_type.into(), &data);
    }

    pub fn set_service_uuid128s(&mut self, uuids: &[uuid::Uuid], is_complete_list: bool) {
        let adv_type = if is_complete_list {
            AdvDataType::Service128bitUuidComplete
        } else {
            AdvDataType::Service128bitUuidMoreAvailable
        };
        let data: Vec<u8> = uuids
            .iter()
            .map(|x| x.as_bytes().to_owned())
            .flatten()
            .collect();
        self.add_entry(adv_type.into(), &data);
    }

    pub fn serialize(&self) -> Vec<u8> {
        // Data is in length-type-value format
        let mut adv_data = Vec::new();
        for (adv_type, data) in self.entries.iter() {
            adv_data.push((data.len() + 1) as u8);
            adv_data.push(*adv_type);
            adv_data.extend(data);
        }

        adv_data
    }

    pub fn validate(&self) -> Result<(), String> {
        let encoded_length = self.serialize().len();
        if encoded_length <= MAX_ADVERTISE_ENCODED_LEN {
            Ok(())
        } else {
            Err(format!("Encoded length too long: {} bytes", encoded_length))
        }
    }
}
