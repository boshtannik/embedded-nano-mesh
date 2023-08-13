use heapless::{String, Vec};

use super::config::CONTENT_SIZE;

pub type PacketString = String<CONTENT_SIZE>;
pub type PacketStringBytes = Vec<u8, CONTENT_SIZE>;

pub type AddressType = u8;
pub type ChecksumType = u8;
pub type FlagsType = u128;
