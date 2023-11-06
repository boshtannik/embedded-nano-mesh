use super::super::traits::{UniqueId, UniqueIdExtractor};
use super::super::Packet;

impl UniqueIdExtractor for Packet {
    fn get_unique_id(&self) -> UniqueId {
        UniqueId::new(self.source_device_identifier.clone(), self.id)
    }
}
