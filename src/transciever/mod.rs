use core::cell::RefCell;

mod config;
mod packet;
mod receiver;
mod transmitter;
mod types;

pub use packet::DeviceIdentifyer;
pub use types::TranscieverString;

use crate::serial_println;

use self::{packet::PacketDataBytes, receiver::ReceiverError, types::PacketQueue};

pub struct Transciever {
    transmitter: transmitter::Transmitter,
    receiver: receiver::Receiver,
}

pub enum TranscieverError {
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
    ) -> Result<(), TranscieverError> {
        match self.transmitter.send(data, destination_device_identifyer) {
            Ok(_) => Ok(()),
            Err(transmitter::TransmitterError::PacketQueueIsFull) => {
                Err(TranscieverError::TryAgainLater)
            }
        }
    }

    pub fn receive(&mut self) -> Option<PacketDataBytes> {
        self.receiver.receive()
    }

    pub fn update(&mut self) {
        match self.receiver.update() {
            Err(ReceiverError::MessageQueueIsFull) => serial_println!("Receiver queue is full"),
            Err(ReceiverError::TransitPacketQueueIsFull) => {
                serial_println!("Transit packet queue is full")
            }
            Err(ReceiverError::NoPacketToManage) => serial_println!("Packet was not received"),
            Ok(_) => (),
        };
        self.transmitter.update();
    }
}
