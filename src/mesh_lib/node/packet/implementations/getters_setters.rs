use super::super::types::{IdType, LifeTimeType};
use super::super::Packet;

impl Packet {
    /// Returns the packet identification number.
    pub fn get_id(&self) -> IdType {
        self.id
    }

    /// Sets the packet identification number.
    pub fn set_id(&mut self, id: IdType) {
        self.id = id;
    }

    /// Increments the packet id, wrapping around on overflow.
    pub fn increment_id(&mut self) {
        self.id = self.id.overflowing_add(1).0;
    }

    /// Returns the remaining packet lifetime (number of hops it can still travel).
    pub fn get_lifetime(&self) -> LifeTimeType {
        self.lifetime
    }
}
