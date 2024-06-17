use heapless::{Deque, String};

use super::packet::{Packet, PACKET_BYTES_SIZE};
use super::PacketMetaData;

use super::constants::{PACKET_META_DATA_QUEUE_SIZE, PACKET_QUEUE_SIZE, PACKET_START_BYTES_COUNT};

pub type PacketQueue = Deque<Packet, PACKET_QUEUE_SIZE>;
pub type PacketDataQueue = Deque<PacketMetaData, PACKET_META_DATA_QUEUE_SIZE>;
pub type ParserBytesBuffer = Deque<u8, { PACKET_BYTES_SIZE + PACKET_START_BYTES_COUNT }>;

/// Type alias for a String with fixed length, that is made
/// to simplify messaging between nodes.
pub type NodeString = String<{ super::packet::CONTENT_SIZE }>;
