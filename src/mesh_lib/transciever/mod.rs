use core::cell::RefCell;

mod config;
mod packet;
mod receiver;
mod special_packet_handler;
mod timer;
mod transmitter;
mod types;

use avr_device::interrupt::Mutex;
pub use packet::DeviceIdentifyer;
pub use special_packet_handler::SpecPacketState;
pub use types::TranscieverString;

pub use packet::LifeTimeType;

use self::{
    packet::{IdType, PacketDataBytes, StateMutator},
    receiver::ReceiverError,
    special_packet_handler::{ErrCase, OkCase, SpecialPacketHandler},
    types::{PacketDataQueue, PacketQueue},
};

use super::millis::{self, ms};

pub static GLOBAL_MUTEXED_CELLED_PACKET_QUEUE: Mutex<RefCell<PacketQueue>> =
    Mutex::new(RefCell::new(PacketQueue::new()));

pub struct Transciever {
    transmitter: transmitter::Transmitter,
    receiver: receiver::Receiver,
    my_address: DeviceIdentifyer,
    timer: timer::Timer,
    received_packet_meta_data_queue: PacketDataQueue,
    special_packet_handler: SpecialPacketHandler,
    received_packet_meta_data: Option<PacketMetaData>,
}

pub enum TranscieverError {
    SendingQueueIsFull,
}

pub enum TranscieverUpdateError {
    NoPacketToManage,
    ReceivingQueueIsFull,
    TransitQueueIsFull,
}

// TODO: Move out to packet
#[derive(Clone)]
pub struct PacketMetaData {
    pub data: PacketDataBytes,
    pub source_device_identifyer: DeviceIdentifyer,
    pub destination_device_identifyer: DeviceIdentifyer,
    pub lifetime: LifeTimeType,
    pub filter_out_duplication: bool, // TODO: Rename in the whole project to void echo, or something...???
    pub packet_spec_state: SpecPacketState,
    pub packet_id: IdType,
}

pub enum PacketMetaDataError {
    PacketLifetimeEnded,
}

impl PacketMetaData {
    fn swap_source_destination(&mut self) {
        (
            self.source_device_identifyer,
            self.destination_device_identifyer,
        ) = (
            DeviceIdentifyer(self.destination_device_identifyer.0),
            DeviceIdentifyer(self.source_device_identifyer.0),
        );
    }
    pub fn deacrease_lifetime(mut self) -> Result<Self, PacketMetaDataError> {
        match self.lifetime.cmp(&1) {
            core::cmp::Ordering::Greater => {
                self.lifetime -= 1;
                Ok(self)
            }
            _ => Err(PacketMetaDataError::PacketLifetimeEnded),
        }
    }

    pub fn is_destination_identifyer_reached(&self, identifyer: &DeviceIdentifyer) -> bool {
        self.destination_device_identifyer == *identifyer
    }
}

impl StateMutator for PacketMetaData {
    fn mutated(mut self) -> Self {
        let old_state = self.packet_spec_state;
        match old_state {
            SpecPacketState::PingPacket
            | SpecPacketState::SendTransaction
            | SpecPacketState::AcceptTransaction
            | SpecPacketState::InitTransaction => self.swap_source_destination(),
            _ => (),
        }
        self
    }
}

pub enum AnswerError {
    ErrorKind(TranscieverError),
    Timeout,
}

impl Transciever {
    pub fn new(my_address: DeviceIdentifyer, listen_period: ms) -> Transciever {
        Transciever {
            transmitter: transmitter::Transmitter::new(),
            receiver: receiver::Receiver::new(),
            my_address,
            timer: timer::Timer::new(listen_period),
            received_packet_meta_data_queue: PacketDataQueue::new(),
            special_packet_handler: SpecialPacketHandler::new(my_address.clone()),
            received_packet_meta_data: None,
        }
    }

    pub fn send_ping_pong(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifyer: DeviceIdentifyer,
        lifetime: LifeTimeType,
        filter_out_duplication: bool,
        timeout: ms,
    ) -> Result<(), AnswerError> {
        let lifetime = if lifetime < 2 { 2 } else { lifetime };

        let mut current_time = millis::millis();
        let wait_end_time = current_time + timeout;

        while let Some(_) = self.receive() {} // Flush out all messages in the queuee.

        match self._send(PacketMetaData {
            data,
            source_device_identifyer: self.my_address.clone(),
            destination_device_identifyer: destination_device_identifyer.clone(),
            lifetime,
            filter_out_duplication,
            packet_spec_state: SpecPacketState::PingPacket,
            packet_id: 0,
        }) {
            Ok(_) => (),
            Err(TranscieverError::SendingQueueIsFull) => {
                return Err(AnswerError::ErrorKind(TranscieverError::SendingQueueIsFull))
            }
        };

        while current_time < wait_end_time {
            self.update();
            if let Some(answer) = self.receive() {
                if answer.source_device_identifyer == destination_device_identifyer {
                    return Ok(());
                }
            }
            current_time = millis::millis();
        }

        Err(AnswerError::Timeout)
    }
    // pub fn send_with_transaction(&mut self, data: PacketDataBytes, destination_device_identifyer:
    // DeviceIdentifyer: DeviceIdentifyer, lifetime: LifeTimeType, filter_out_duplications: bool) ->
    // Result<(), TransactionFailed>

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
        filter_out_duplication: bool,
    ) -> Result<(), TranscieverError> {
        self._send(PacketMetaData {
            data,
            source_device_identifyer: self.my_address.clone(),
            destination_device_identifyer,
            lifetime,
            filter_out_duplication,
            packet_spec_state: SpecPacketState::Normal,
            packet_id: 0,
        })
    }

    fn _send(&mut self, packet_meta_data: PacketMetaData) -> Result<(), TranscieverError> {
        match self.transmitter.send(packet_meta_data) {
            Ok(_) => Ok(()),
            Err(transmitter::TransmitterError::PacketQueueIsFull) => {
                Err(TranscieverError::SendingQueueIsFull)
            }
        }
    }

    /// Optionally returns `PacketDataBytes` instance with data,
    /// which has been send exactly to this device, or has been
    /// `broadcast`ed trough all the network.
    pub fn receive(&mut self) -> Option<PacketMetaData> {
        self.receiver.receive()
    }

    /// Does all necessary internal work of mesh node:
    /// * Receives packets from ether, and manages their further life.
    ///     ** Data of other devices are going to be send back into ether.
    ///     ** Data addressed to current device, will be unpacked and stored.
    pub fn update(&mut self) -> Result<(), TranscieverUpdateError> {
        if self.timer.is_time_to_speak() {
            self.transmitter.update();
            self.timer.record_speak_time();
        }
        match self.receiver.update() {
            Err(ReceiverError::NoPacketToManage) => (),
            Err(ReceiverError::PacketDuplication) => (),
            Err(ReceiverError::DuplicationFilterOverloaded) => (),
            Ok(_) => (),
        };

        let packet_to_handle = match self.receiver.receive() {
            Some(packet_to_handle) => packet_to_handle,
            None => return Err(TranscieverUpdateError::NoPacketToManage),
        };

        let reached_destination_packet = match self.special_packet_handler.handle(packet_to_handle)
        {
            Ok(ok_case) => match ok_case {
                OkCase::Handled => return Ok(()),
                OkCase::DestinationReached(packet) => packet,
            },
            Err(err_case) => match err_case {
                ErrCase::TransitQueueIsFull => {
                    return Err(TranscieverUpdateError::TransitQueueIsFull)
                }
                ErrCase::PacketLifetimeEnded => return Ok(()),
            },
        };

        match self
            .received_packet_meta_data_queue
            .push_back(reached_destination_packet)
        {
            Ok(()) => Ok(()),
            Err(_) => Err(TranscieverUpdateError::ReceivingQueueIsFull),
        }
    }
}
