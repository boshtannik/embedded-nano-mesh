use super::{
    types::{AddressType, ChecksumType, DataLengthType, FlagsType, IdType, LifeTimeType},
    Packet,
};
use core::mem::size_of;

/// Size of the content of the packet.
pub const CONTENT_SIZE: usize = 32;

/// Size of the packet in bytes.
pub const PACKET_BYTES_SIZE: usize = Packet::size_of_bytes();

/// This flag make the device, which have packet caught, to ignore
/// same packets, which were re-transmitted from other devices.
pub const IGNORE_DUPLICATIONS_FLAG: FlagsType = 0b10000000;

pub const ADDRESS_TYPE_SIZE: usize = size_of::<AddressType>();
pub const ID_TYPE_SIZE: usize = size_of::<IdType>();
pub const LIFETIME_TYPE_SIZE: usize = size_of::<LifeTimeType>();
pub const FLAGS_TYPE_SIZE: usize = size_of::<FlagsType>();
pub const DATA_LENGTH_TYPE_SIZE: usize = size_of::<DataLengthType>();
pub const DATA_TYPE_SIZE: usize = CONTENT_SIZE;
pub const CHECKSUM_TYPE_SIZE: usize = size_of::<ChecksumType>();
