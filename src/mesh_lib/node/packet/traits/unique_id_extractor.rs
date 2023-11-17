use super::super::{AddressType, IdType};

#[derive(PartialEq, Eq, Clone)]
pub struct UniqueId(AddressType, IdType);

impl UniqueId {
    pub fn new(source_device_identifier: AddressType, id: IdType) -> UniqueId {
        UniqueId(source_device_identifier, id)
    }
}

/// This trait is made for being able to tell instances one
/// from another. It builds values of fields combination, which shall
/// be extremely rare to be accidentally occurred.
/// Is used to identify same packet in the network.
pub trait UniqueIdExtractor {
    /// builds and returns UniquePacketId of packet.
    fn get_unique_id(&self) -> UniqueId;
}
