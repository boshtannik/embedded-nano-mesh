use heapless::{Deque, String};

use super::packet::{Packet, PacketDataBytes, PACKET_BYTES_SIZE};

use super::config::QUEUE_SIZE;

pub type PacketQueue = Deque<Packet, QUEUE_SIZE>;
pub type PacketDataQueue = Deque<PacketDataBytes, 10>;

pub type PacketBytesBuffer = Deque<u8, { PACKET_BYTES_SIZE + 10 }>;

pub type TranscieverString = String<{ super::packet::CONTENT_SIZE }>;
