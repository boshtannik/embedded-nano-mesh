use core::cell::RefCell;

use heapless::Deque;

use crate::packet::{Packet, String64};

type MessageQueue = Deque<String64, 10>;
pub type PacketQueue = Deque<Packet, 10>;

pub struct Transmitter {
    message_queue: MessageQueue,
    packet_queue: PacketQueue,
    transit_packet_queue: RefCell<PacketQueue>,
}

enum Error {
    PacketQueueIsFull,
    MessageQueueIsFull,
}

impl Transmitter {
    pub fn new(transit_packet_queue: RefCell<PacketQueue>) -> Transmitter {
        Transmitter {
            message_queue: MessageQueue::new(),
            packet_queue: PacketQueue::new(),
            transit_packet_queue,
        }
    }

    pub fn send_message(&mut self, item: String64) -> Result<(), Error> {
        match self.message_queue.push_back(item) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::MessageQueueIsFull),
        }
    }

    pub fn update(&mut self) {
        // Pack messages into packets
        // In case of sending time has come -> Send packets over serial
    }
}
