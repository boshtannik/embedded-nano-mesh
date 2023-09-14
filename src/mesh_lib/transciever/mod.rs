use core::cell::RefCell;

mod config;
mod packet;
mod receiver;
mod timer;
mod transmitter;
mod types;

use avr_device::interrupt::Mutex;
pub use packet::DeviceIdentifyer;
pub use types::TranscieverString;

pub use packet::LifeTimeType;

use self::{packet::PacketDataBytes, receiver::ReceiverError, types::PacketQueue};

use super::millis::ms;

pub static GLOBAL_MUTEXED_CELLED_QUEUE: Mutex<RefCell<PacketQueue>> =
    Mutex::new(RefCell::new(PacketQueue::new()));

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
        Transciever {
            transmitter: transmitter::Transmitter::new(my_address.clone()),
            receiver: receiver::Receiver::new(my_address),
            timer: timer::Timer::new(listen_period),
        }
    }

    pub fn send(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifyer: DeviceIdentifyer,
        lifetime: LifeTimeType,
    ) -> Result<(), TranscieverError> {
        match self
            .transmitter
            .send(data, destination_device_identifyer, lifetime)
        {
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
            Err(ReceiverError::TransitPacketLifetimeEnded) => {}
            Err(ReceiverError::NoPacketToManage) => (),
            Err(ReceiverError::PacketDuplication) => (),
            Ok(_) => (),
        };
    }
}
