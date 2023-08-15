use heapless::Deque;

use super::packet::{Packet, PacketString};

use super::config::QUEUE_SIZE;

pub type MessageQueue = Deque<PacketString, QUEUE_SIZE>;
pub type PacketQueue = Deque<Packet, QUEUE_SIZE>;
