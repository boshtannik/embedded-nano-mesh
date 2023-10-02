use crate::mesh_lib::node::PacketMetaData;

use super::{
    types::{IdType, PacketSerializedBytes},
    DeviceIdentifyer, Packet,
};

pub trait DataPacker {
    fn pack(packet_meta_data: PacketMetaData) -> Packet;
    fn unpack(self: Self) -> PacketMetaData;
}

pub trait PacketFlagOps {
    // IGNORE_DUPLICATION_FLAG
    fn set_ignore_duplication_flag(&mut self, new_state: bool);
    fn is_ignore_duplication_flag_set(&self) -> bool;

    // PING_FLAG
    fn set_ping_flag(&mut self, new_state: bool);
    fn is_ping_flag_set(&self) -> bool;

    // PONG_FLAG
    fn set_pong_flag(&mut self, new_state: bool);
    fn is_pong_flag_set(&self) -> bool;

    // SEND_TRANSACTION_FLAG
    fn set_send_transaction_flag(&mut self, new_state: bool);
    fn is_send_transaction_flag_set(&self) -> bool;

    // ACCEPT_TRANSACTION_FLAG
    fn set_accept_transaction_flag(&mut self, new_state: bool);
    fn is_accept_transaction_flag_set(&self) -> bool;

    // INITIATE_TRANSACTION_FLAG
    fn set_initiate_transaction_flag(&mut self, new_state: bool);
    fn is_initiate_transaction_flag_set(&self) -> bool;

    // FINISH_TRANSACTION_FLAG
    fn set_finish_transaction_flag(&mut self, new_state: bool);
    fn is_finish_transaction_flag_set(&self) -> bool;
}

pub trait Serializer {
    fn serialize(self) -> PacketSerializedBytes;
    fn deserialize(bytes: PacketSerializedBytes) -> Self;
}

pub trait StateMutator {
    fn mutated(self) -> Self;
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
