use super::constants::{
    ADDRESS_TYPE_SIZE, CONTENT_SIZE, DATA_LENGTH_TYPE_SIZE, PACKET_BYTES_COUNT,
};

use heapless::Vec;

use super::FromBytes;

/// Type alias for packet identification number.
pub type IdType = u8;

/// Type alias for packet bit flags.
pub type FlagsType = u8;

/// Type alias for packet address identification number.
pub type AddressType = u8;

/// Type alias for packet checksum.
pub type ChecksumType = u8;

/// Type alias for packet lifetime. This value contains the information,
/// about for how many times the packet can be re-sent.
pub type LifeTimeType = u8;

/// Type alias for data contained in the packet.
pub type PacketDataBytes = Vec<u8, { CONTENT_SIZE }>;

/// Type alias that represents serialized packet bytes sequence.
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

/// State of the packet.
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
