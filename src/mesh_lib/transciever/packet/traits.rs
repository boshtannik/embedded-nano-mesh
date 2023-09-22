use super::{
    types::{IdType, LifeTimeType, PacketDataBytes, PacketSerializedBytes},
    DeviceIdentifyer, Packet,
};

pub trait DataPacker {
    fn pack(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        id: IdType,
        lifetime: LifeTimeType,
        data: PacketDataBytes,
    ) -> Packet;

    fn unpack(self: Self) -> PacketDataBytes;
}

pub trait PacketFlagOps {
    // SEND_TRANSACTION_FLAG
    // ACCEPT_TRANSACTION_FLAG
    // INITIATE_TRANSACTION_FLAG
    // FINISH_TRANSACTION_FLAG
    fn set_ignore_duplication_flag(&mut self, new_state: bool);
    fn is_ignore_duplication_flag_set(&self) -> bool;

    fn set_require_answer_flag(&mut self, new_state: bool);
    fn is_require_answer_flag_set(&self) -> bool;

    fn set_provide_answer_flag(&mut self, new_state: bool);
    fn is_provide_answer_flag_set(&self) -> bool;
}

pub trait Serializer {
    fn serialize(self) -> PacketSerializedBytes;
    fn deserialize(bytes: PacketSerializedBytes) -> Self;
}

#[derive(PartialEq, Eq, Clone)]
pub struct UniqueId(DeviceIdentifyer, IdType);

impl UniqueId {
    pub fn new(source_device_identifyer: DeviceIdentifyer, id: IdType) -> UniqueId {
        UniqueId(source_device_identifyer, id)
    }
}

/// This strait is made for being eble to tell instances one
/// from another. It builds values of fields combination, which shall
/// be extremely rare to be accidentally duplicated.
/// Is used to identify same packet in the network.
pub trait UniqueIdExtractor {
    /// builds and returns UniquePacketId of packet.
    fn get_unique_id(&self) -> UniqueId;
}

pub trait FromBytes<const TYPE_SIZE: usize> {
    fn from_be_bytes(bytes: [u8; TYPE_SIZE]) -> Self;
}
