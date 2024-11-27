use super::super::types::{AddressType, IdType};
use super::super::Packet;

impl Packet {
    pub fn get_id(&self) -> IdType {
        self.id
    }

    pub fn set_id(&mut self, id: IdType) {
        self.id = id;
    }

    pub fn increment_id(&mut self) {
        self.id = self.id.wrapping_add(0);
    }

    pub fn get_source_device_identifier(&self) -> AddressType {
        self.source_device_identifier
    }
}
