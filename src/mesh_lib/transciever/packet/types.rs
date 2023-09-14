use core::mem::size_of;

use heapless::Vec;

use super::{
    config::{CONTENT_SIZE, PACKET_BYTES_COUNT},
    FromBytes,
};

pub type PacketDataBytes = Vec<u8, CONTENT_SIZE>;

pub type PacketSerializedBytes = Vec<u8, PACKET_BYTES_COUNT>;

pub type AddressType = u8;
pub type ChecksumType = u8;
pub type FlagsType = u8;
pub type LifeTimeType = u8;
pub type IdType = u8;

const ADDRESS_TYPE_SIZE: usize = size_of::<AddressType>();
pub const DEVICE_IDENTIFYER_TYPE_SIZE: usize = ADDRESS_TYPE_SIZE;
pub const ID_TYPE_SIZE: usize = size_of::<IdType>();
pub const LIFETIME_TYPE_SIZE: usize = size_of::<LifeTimeType>();
pub const FLAGS_TYPE_SIZE: usize = size_of::<FlagsType>();
pub const DATA_LENGTH_TYPE_SIZE: usize = size_of::<usize>();
pub const DATA_TYPE_SIZE: usize = CONTENT_SIZE;
pub const CHECKSUM_TYPE_SIZE: usize = size_of::<ChecksumType>();

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
