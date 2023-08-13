use heapless::Deque;

use crate::packet::{Packet, String64};

pub type MessageQueue = Deque<String64, 10>;
pub type PacketQueue = Deque<Packet, 10>;
