use super::{types::AddressType, Packet};

pub const CONTENT_SIZE: usize = 32;
pub const PACKET_BYTES_COUNT: usize = Packet::size_of_bytes();

pub const BROADCAST_RESERVED_IDENTIFYER: AddressType = 0;
