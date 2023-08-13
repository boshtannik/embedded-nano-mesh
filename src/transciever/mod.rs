use core::cell::RefCell;

use crate::packet::DeviceIdentifyer;

mod config;
mod receiver;
mod transmitter;
mod types;

use receiver::Receiver;
use transmitter::Transmitter;

use self::types::PacketQueue;

pub struct Transciever {
    transmitter: Transmitter,
    receiver: Receiver,
}

pub enum Error {
    SendError,
}

impl Transciever {
    pub fn new(my_address: DeviceIdentifyer) -> Transciever {
        let mut transit_packet_queue: RefCell<PacketQueue> = RefCell::new(PacketQueue::new());
        Transciever {
            transmitter: Transmitter::new(
                my_address.clone(),
                RefCell::clone(&transit_packet_queue),
            ),
            receiver: Receiver::new(my_address.clone(), RefCell::clone(&transit_packet_queue)),
        }
    }

    /*
    pub fn send_message(
        &mut self,
        message: String64,
        destination_device_identifyer: DeviceIdentifyer,
    ) -> Result<(), Error> {
        match self
            .transmitter
            .send_message(message, destination_device_identifyer)
        {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::SendError),
        }
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
    */
}
