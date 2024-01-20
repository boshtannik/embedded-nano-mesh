use super::super::traits::{PacketUniqueId, UniqueIdExtractor};
use super::super::Packet;

impl UniqueIdExtractor for Packet {
    fn get_unique_id(&self) -> PacketUniqueId {
        PacketUniqueId::new(self.source_device_identifier.clone(), self.id)
    }
}
