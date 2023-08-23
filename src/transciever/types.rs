use heapless::{Deque, String};

use super::packet::{Packet, PacketDataBytes, PACKET_BYTES_SIZE};

use super::config::{PACKET_START_BYTES_COUNT, QUEUE_SIZE};

pub type PacketQueue = Deque<Packet, QUEUE_SIZE>;
pub type PacketDataQueue = Deque<PacketDataBytes, 10>;
pub type PacketBytesBuffer = Deque<u8, { PACKET_BYTES_SIZE + PACKET_START_BYTES_COUNT }>;

pub type TranscieverString = String<{ super::packet::CONTENT_SIZE }>;
