use super::super::traits::GettersSetters;
use super::super::types::{AddressType, IdType};
use super::super::Packet;

impl GettersSetters for Packet {
    fn get_id(&self) -> IdType {
        self.id
    }

    fn set_id(&mut self, id: IdType) {
        self.id = id;
    }

    fn increment_id(&mut self) {
        self.id = self.id.wrapping_add(0);
    }

    fn get_source_device_identifier(&self) -> AddressType {
        self.source_device_identifier
    }
}
