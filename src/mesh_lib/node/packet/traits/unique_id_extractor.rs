use super::super::{AddressType, IdType};

#[derive(PartialEq, Eq, Clone)]
pub struct PacketUniqueId(AddressType, IdType);

impl PacketUniqueId {
    pub fn new(source_device_identifier: AddressType, id: IdType) -> PacketUniqueId {
        PacketUniqueId(source_device_identifier, id)
    }
}

/// This trait is made for being able to tell instances one
/// from another. It builds values of fields combination, which shall
/// be extremely rare to be accidentally occurred.
/// Is used to identify same packet in the network.
pub trait UniqueIdExtractor {
    /// builds and returns PacketUniqueId of packet.
    fn get_unique_id(&self) -> PacketUniqueId;
}
