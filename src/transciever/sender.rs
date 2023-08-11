use heapless::Deque;

use crate::packet::{Packet, String64};

type MessageQueue = Deque<String64, 10>;
type PacketQueue = Deque<Packet, 10>;

pub struct Transmitter {
    messages_queue: MessageQueue,
    packets_queue: PacketQueue,
}

enum Error {
    PacketsQueueIsFull,
    MessagesQueueIsFull,
}

impl Transmitter {
    pub fn new() -> Transmitter {
        Transmitter {
            messages_queue: MessageQueue::new(),
            packets_queue: PacketQueue::new(),
        }
    }

    pub fn send_message(&mut self, item: String64) -> Result<(), Error> {
        match self.messages_queue.push_back(item) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::MessagesQueueIsFull),
        }
    }
}
