use core::cell::RefCell;

use crate::packet::DeviceAddress;

mod receiver;
mod transmitter;
mod types;

use receiver::Receiver;
use transmitter::Transmitter;

use crate::packet::String64;

use self::types::{MessageQueue, PacketQueue};

pub struct Transciever {
    my_address: DeviceAddress,
    transmitter: Transmitter,
    receiver: Receiver,
}

enum Error {
    Send,
}

impl Transciever {
    pub fn new(my_address: DeviceAddress) -> Transciever {
        let mut transit_packet_queue: RefCell<PacketQueue> = RefCell::new(PacketQueue::new());
        Transciever {
            my_address,
            transmitter: Transmitter::new(RefCell::clone(&transit_packet_queue)),
            receiver: Receiver::new(RefCell::clone(&transit_packet_queue)),
        }
    }

    pub fn send_message(
        &mut self,
        message: String64,
        target_address: DeviceAddress,
    ) -> Result<(), Error> {
        // Split message into pieces, or not?
        Err(Error::Send)
    }

    pub fn update(&mut self) {
        self.receiver.update();
        // Iterate over received messages:
        //     In case if message is addressed to other Transciever -> move it into transit_packet_queue
        self.transmitter.update();
    }

    pub fn received_messages(&mut self) -> MessageQueue {
        self.receiver.received_messages()
    }
}
