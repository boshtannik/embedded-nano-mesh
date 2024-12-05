use super::super::types::IdType;
use super::super::Packet;

impl Packet {
    pub fn get_id(&self) -> IdType {
        self.id
    }

    pub fn set_id(&mut self, id: IdType) {
        self.id = id;
    }

    pub fn increment_id(&mut self) {
        self.id = self.id.overflowing_add(1).0;
    }
}
