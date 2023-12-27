use heapless::{Deque, String};

use super::packet::{Packet, PACKET_BYTES_COUNT};
use super::PacketMetaData;

use super::constants::{PACKET_START_BYTES_COUNT, QUEUE_SIZE};

pub type PacketQueue = Deque<Packet, QUEUE_SIZE>;
pub type PacketDataQueue = Deque<PacketMetaData, 10>;
pub type PacketBytesBuffer = Deque<u8, { PACKET_BYTES_COUNT + PACKET_START_BYTES_COUNT }>;

/// Type alias for a String with fixed length, that is made
/// to unify use of the library.
pub type NodeString = String<{ super::packet::CONTENT_SIZE }>;
