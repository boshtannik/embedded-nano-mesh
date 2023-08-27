use heapless::Vec;

use super::config::{CONTENT_SIZE, PACKET_BYTES_SIZE};

pub type PacketDataBytes = Vec<u8, CONTENT_SIZE>;

pub type PacketSerializedBytes = Vec<u8, PACKET_BYTES_SIZE>;

pub type AddressType = u8;
pub type ChecksumType = u8;
pub type FlagsType = u8;
pub type LifeTimeType = u8;
