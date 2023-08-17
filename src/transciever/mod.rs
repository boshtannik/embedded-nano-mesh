use core::cell::RefCell;

mod config;
mod packet;
mod receiver;
mod transmitter;
mod types;

pub use packet::DeviceIdentifyer;
pub use types::TranscieverString;

use self::{packet::PacketDataBytes, types::PacketQueue};

pub struct Transciever {
    transmitter: transmitter::Transmitter,
    receiver: receiver::Receiver,
}

pub enum Error {
    TryAgainLater,
}

impl Transciever {
    pub fn new(my_address: DeviceIdentifyer) -> Transciever {
        let transit_packet_queue: RefCell<PacketQueue> = RefCell::new(PacketQueue::new());
        Transciever {
            transmitter: transmitter::Transmitter::new(
                my_address.clone(),
                RefCell::clone(&transit_packet_queue),
            ),
            receiver: receiver::Receiver::new(
                my_address.clone(),
                RefCell::clone(&transit_packet_queue),
            ),
        }
    }

    pub fn send(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifyer: DeviceIdentifyer,
    ) -> Result<(), Error> {
        match self.transmitter.send(data, destination_device_identifyer) {
            Ok(_) => Ok(()),
            Err(transmitter::Error::PacketQueueIsFull) => Err(Error::TryAgainLater),
        }
    }

    pub fn receive(&mut self) -> Option<PacketDataBytes> {
        self.receiver.receive()
    }

    pub fn update(&mut self) {
        self.receiver.update();
        self.transmitter.update();
    }
}
