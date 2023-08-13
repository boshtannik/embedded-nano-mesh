use core::cell::RefCell;

use heapless::Deque;

use crate::packet::{Packet, String64};

pub type MessageQueue = Deque<String64, 10>;
pub type PacketQueue = Deque<Packet, 10>;

pub struct Receiver {
    message_queue: MessageQueue,
    packet_queue: PacketQueue,
    transit_packet_queue: RefCell<PacketQueue>,
}

enum Error {
    PacketQueueIsFull,
    MessageQueueIsFull,
}

impl Receiver {
    pub fn new(transit_packet_queue: RefCell<PacketQueue>) -> Receiver {
        Receiver {
            message_queue: MessageQueue::new(),
            packet_queue: PacketQueue::new(),
            transit_packet_queue,
        }
    }

    pub fn update(&mut self) {
        // Check received packets.
        // In case if packet is corrupt -> drop it.
        //
        // In case if packet is ok:
        //      If location is reached - Move out message into message queue.
        //      If location is other - Move packet into transit_packet_queue.
    }

    pub fn received_messages(&self) -> MessageQueue {
        self.message_queue.clone()
    }
}
