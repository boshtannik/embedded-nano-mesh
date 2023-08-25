use core::mem::size_of;

use super::Packet;

pub const CONTENT_SIZE: usize = 32;
pub const PACKET_BYTES_SIZE: usize = size_of::<Packet>();
