use core::cell::RefCell;

mod config;
mod packet;
mod receiver;
mod timer;
mod transmitter;
mod types;

pub use packet::DeviceIdentifyer;
pub use types::TranscieverString;

use crate::millis::ms;

use self::{packet::PacketDataBytes, receiver::ReceiverError, types::PacketQueue};

pub struct Transciever {
    transmitter: transmitter::Transmitter,
    receiver: receiver::Receiver,
    timer: timer::Timer,
}

pub enum TranscieverError {
    TryAgainLater,
}

impl Transciever {
    pub fn new(my_address: DeviceIdentifyer, listen_period: ms) -> Transciever {
        let transit_packet_queue: RefCell<PacketQueue> = RefCell::new(PacketQueue::new());
        Transciever {
            transmitter: transmitter::Transmitter::new(
                my_address.clone(),
                RefCell::clone(&transit_packet_queue),
            ),
            receiver: receiver::Receiver::new(my_address, transit_packet_queue),
            timer: timer::Timer::new(listen_period),
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
        if self.timer.is_time_to_speak() {
            self.transmitter.update();
            self.timer.record_speak_time();
        }
        match self.receiver.update() {
            Err(ReceiverError::MessageQueueIsFull) => {}
            Err(ReceiverError::TransitPacketQueueIsFull) => {}
            Err(ReceiverError::NoPacketToManage) => (),
            Ok(_) => (),
        };
    }
}
