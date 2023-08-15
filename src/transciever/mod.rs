use core::cell::RefCell;

mod config;
mod packet;
mod receiver;
mod transmitter;
mod types;

pub use packet::{DeviceIdentifyer, PacketString};

use receiver::Receiver;
use transmitter::Transmitter;

use self::types::PacketQueue;

pub struct Transciever {
    transmitter: Transmitter,
    receiver: Receiver,
}

pub enum Error {
    TryAgainLater,
}

impl Transciever {
    pub fn new(my_address: DeviceIdentifyer) -> Transciever {
        let transit_packet_queue: RefCell<PacketQueue> = RefCell::new(PacketQueue::new());
        Transciever {
            transmitter: Transmitter::new(
                my_address.clone(),
                RefCell::clone(&transit_packet_queue),
            ),
            receiver: Receiver::new(my_address.clone(), RefCell::clone(&transit_packet_queue)),
        }
    }

    pub fn send_message(
        &mut self,
        message: PacketString,
        destination_device_identifyer: DeviceIdentifyer,
    ) -> Result<(), Error> {
        match self
            .transmitter
            .send_message(message, destination_device_identifyer)
        {
            Ok(_) => Ok(()),
            Err(transmitter::Error::PacketQueueIsFull) => Err(Error::TryAgainLater),
        }
    }

    pub fn update(&mut self) {
        self.receiver.update();
        self.transmitter.update();
    }

    /*
    pub fn received_messages(&mut self) -> MessageQueue {
        self.receiver.received_messages()
    }
    */
}
