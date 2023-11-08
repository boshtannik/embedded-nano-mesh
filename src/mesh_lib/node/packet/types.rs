use super::constants::{
    ADDRESS_TYPE_SIZE, CONTENT_SIZE, DATA_LENGTH_TYPE_SIZE, PACKET_BYTES_COUNT,
};

use heapless::Vec;

use super::FromBytes;

pub type IdType = u8;
pub type FlagsType = u8;
pub type AddressType = u8;
pub type ChecksumType = u8;
pub type LifeTimeType = u8;

pub type PacketDataBytes = Vec<u8, { CONTENT_SIZE }>;
pub type PacketSerializedBytes = Vec<u8, { PACKET_BYTES_COUNT }>;

impl FromBytes<ADDRESS_TYPE_SIZE> for AddressType {
    fn from_be_bytes(bytes: [u8; ADDRESS_TYPE_SIZE]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

impl FromBytes<DATA_LENGTH_TYPE_SIZE> for usize {
    fn from_be_bytes(bytes: [u8; DATA_LENGTH_TYPE_SIZE]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum PacketState {
    Normal,
    Ping,
    Pong,
    SendTransaction,
    AcceptTransaction,
    InitTransaction,
    FinishTransaction,
}

#[derive(Clone, PartialEq, Eq)]
pub struct DeviceIdentifier(pub AddressType);
