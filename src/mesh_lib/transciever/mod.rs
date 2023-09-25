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

pub use packet::{LifeTimeType, BROADCAST_RESERVED_IDENTIFYER};

use self::{packet::PacketDataBytes, receiver::ReceiverError, types::PacketQueue};

use super::millis::ms;

pub static GLOBAL_MUTEXED_CELLED_PACKET_QUEUE: Mutex<RefCell<PacketQueue>> =
    Mutex::new(RefCell::new(PacketQueue::new()));

pub struct Transciever {
    transmitter: transmitter::Transmitter,
    receiver: receiver::Receiver,
    timer: timer::Timer,
}

pub enum TranscieverError {
    SendingQueueIsFull,
}

// pub struct WaitingForAnswerTimeOut;
// pub struct TransactionFailed;

impl Transciever {
    pub fn new(my_address: DeviceIdentifyer, listen_period: ms) -> Transciever {
        Transciever {
            transmitter: transmitter::Transmitter::new(my_address.clone()),
            receiver: receiver::Receiver::new(my_address),
            timer: timer::Timer::new(listen_period),
        }
    }

    // pub fn send_and_wait_answer(&mut self, data: PacketDataBytes, destination_device_identifyer: DeviceIdentifyer: DeviceIdentifyer, lifetime: LifeTimeType, filter_out_duplications: bool) -> Result<(), WaitingForAnswerTimeOut>
    // pub fn send_with_transaction(&mut self, data: PacketDataBytes, destination_device_identifyer: DeviceIdentifyer: DeviceIdentifyer, lifetime: LifeTimeType, filter_out_duplications: bool) -> Result<(), TransactionFailed>

    /// Sends the `data` to exact device. or to all devices.
    /// In order to send `data` to all devices, use `BROADCAST_RESERVED_IDENTIFYER`,
    /// otherwise - use identifyer of exact device, which is not `BROADCAST_RESERVED_IDENTIFYER`
    /// identifyer.
    ///
    /// * `data` - Is the instance of `PacketDataBytes`, which is just type alias of
    /// heapless vector of bytes of special size. This size is configured in the
    /// transciever/packet/config.rs file, and can be adjusted for case of other data size is needed.
    /// `Note!` That all devices should have same version of protocol flashed, in order to
    /// be able to correctly to communicate with each other.
    ///
    /// * `destination_device_identifyer` is instance of DeviceIdentifyer type,
    /// That type is made for simplicity of reading the code, and to strict possible mess-ups
    /// during the usage of methods. It is made to present device id within the network.
    /// `Note!`, that you can send message to all devices at once.
    /// The reason of that, that in this protocol - there is reserved `BROADCAST_RESERVED_IDENTIFYER`.
    /// This is the special kind of identifyer, made especially to make every node
    /// to be able to recognize this identifyer as it's own identifyer. In other words, every node
    /// will receive the broadcast message.
    ///
    /// `lifetime` - is the instance of `LifeTimeType`. This value configures the count of
    /// how many nodes - the packet will be able to pass. Also this value is needed
    /// to void the ether being jammed by packets, that in theory might be echoed
    /// by the nodes to the infinity...
    /// Each device, once passes transit packet trough it - it reduces packet's lifetime.
    pub fn send(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifyer: DeviceIdentifyer,
        lifetime: LifeTimeType,
        filter_out_duplications: bool,
    ) -> Result<(), TranscieverError> {
        match self.transmitter.send(
            data,
            destination_device_identifyer,
            lifetime,
            filter_out_duplications,
        ) {
            Ok(_) => Ok(()),
            Err(transmitter::TransmitterError::PacketQueueIsFull) => {
                Err(TranscieverError::SendingQueueIsFull)
            }
        }
    }

    /// Optionally returns `PacketDataBytes` instance with data,
    /// which has been send exactly to this device, or has been
    /// `broadcast`ed trough all the network.
    pub fn receive(&mut self) -> Option<PacketDataBytes> {
        self.receiver.receive()
    }

    /// Does all necessary internal work of mesh node:
    /// * Receives packets from ether, and manages their further life.
    ///     ** Data of other devices are going to be send back into ether.
    ///     ** Data addressed to current device, will be unpacked and stored.
    pub fn update(&mut self) {
        if self.timer.is_time_to_speak() {
            self.transmitter.update();
            self.timer.record_speak_time();
        }
        match self.receiver.update() {
            Err(ReceiverError::MessageQueueIsFull) => (),
            Err(ReceiverError::TransitPacketQueueIsFull) => (),
            Err(ReceiverError::TransitPacketLifetimeEnded) => (),
            Err(ReceiverError::NoPacketToManage) => (),
            Err(ReceiverError::PacketDuplication) => (),
            Err(ReceiverError::FilterOverloaded) => (),
            Ok(_) => (),
        };
    }
}
