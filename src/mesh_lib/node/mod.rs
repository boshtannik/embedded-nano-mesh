use core::cell::RefCell;

mod config;
mod packet;
mod receiver;
mod router;
mod timer;
mod transmitter;
mod types;

use avr_device::interrupt::Mutex;
pub use packet::DeviceIdentifier;
pub use router::SpecState;
pub use types::NodeString;

pub use packet::LifeTimeType;

use self::{
    packet::{IdType, PacketDataBytes, StateMutator, BROADCAST_RESERVED_IDENTIFIER},
    router::{ErrCase, OkCase, PacketRouter},
    types::{PacketDataQueue, PacketQueue},
};

use super::millis::{self, ms};

pub static GLOBAL_MUTEXED_CELLED_PACKET_QUEUE: Mutex<RefCell<PacketQueue>> =
    Mutex::new(RefCell::new(PacketQueue::new()));

pub struct Node {
    transmitter: transmitter::Transmitter,
    receiver: receiver::Receiver,
    my_address: DeviceIdentifier,
    timer: timer::Timer,
    received_packet_meta_data_queue: PacketDataQueue,
    packet_router: PacketRouter,
}

pub enum NodeError {
    SendingQueueIsFull,
}

pub enum NodeUpdateError {
    ReceivingQueueIsFull,
    TransitQueueIsFull,
}

// TODO: Move out to packet
#[derive(Clone)]
pub struct PacketMetaData {
    pub data: PacketDataBytes,
    pub source_device_identifier: DeviceIdentifier,
    pub destination_device_identifier: DeviceIdentifier,
    pub lifetime: LifeTimeType,
    pub filter_out_duplication: bool, // TODO: Rename in the whole project to void echo, or something...???
    pub spec_state: SpecState,
    pub packet_id: IdType,
}

pub enum PacketMetaDataError {
    PacketLifetimeEnded,
}

impl PacketMetaData {
    fn swap_source_destination(&mut self) {
        (
            self.source_device_identifier,
            self.destination_device_identifier,
        ) = (
            DeviceIdentifier(self.destination_device_identifier.0),
            DeviceIdentifier(self.source_device_identifier.0),
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

    pub fn is_destination_identifier_reached(&self, identifier: &DeviceIdentifier) -> bool {
        self.destination_device_identifier == *identifier
    }
}

impl StateMutator for PacketMetaData {
    fn mutated(mut self) -> Self {
        let old_state = self.spec_state.clone();
        match old_state {
            SpecState::PingPacket
            | SpecState::SendTransaction
            | SpecState::AcceptTransaction
            | SpecState::InitTransaction => self.swap_source_destination(),
            _ => (),
        }
        self.spec_state = old_state.mutated();
        self
    }
}

pub enum SpecialSendError {
    BroadcastAddressForbidden,
    TryAgainLater,
    Timeout,
}

impl Node {
    pub fn new(my_address: DeviceIdentifier, listen_period: ms) -> Node {
        Node {
            transmitter: transmitter::Transmitter::new(),
            receiver: receiver::Receiver::new(),
            my_address: my_address.clone(),
            timer: timer::Timer::new(listen_period),
            received_packet_meta_data_queue: PacketDataQueue::new(),
            packet_router: PacketRouter::new(my_address),
        }
    }

    pub fn send_ping_pong(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifier: DeviceIdentifier,
        lifetime: LifeTimeType,
        filter_out_duplication: bool,
        timeout: ms,
    ) -> Result<(), SpecialSendError> {
        if destination_device_identifier.0 == BROADCAST_RESERVED_IDENTIFIER {
            return Err(SpecialSendError::BroadcastAddressForbidden);
        }
        let mut current_time = millis::millis();
        let wait_end_time = current_time + timeout;

        while let Some(_) = self.receive() {} // Flush out all messages in the queuee.

        match self._send(PacketMetaData {
            data,
            source_device_identifier: self.my_address.clone(),
            destination_device_identifier: destination_device_identifier.clone(),
            lifetime,
            filter_out_duplication,
            spec_state: SpecState::PingPacket,
            packet_id: 0,
        }) {
            Ok(_) => (),
            Err(NodeError::SendingQueueIsFull) => {
                return Err(SpecialSendError::TryAgainLater);
            }
        };

        while current_time < wait_end_time {
            let _ = self.update();

            if let Some(answer) = self.receive() {
                if !(answer.spec_state == SpecState::PongPacket) {
                    continue;
                }
                if !(answer.source_device_identifier == destination_device_identifier) {
                    continue;
                }
                return Ok(());
            }
            current_time = millis::millis();
        }

        Err(SpecialSendError::Timeout)
    }

    // pub fn send_with_transaction(&mut self, data: PacketDataBytes, destination_device_identifier:
    // Deviceidentifier: Deviceidentifier, lifetime: LifeTimeType, filter_out_duplications: bool) ->
    // Result<(), TransactionFailed>
    pub fn send_with_transaction(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifier: DeviceIdentifier,
        lifetime: LifeTimeType,
        filter_out_duplication: bool,
        timeout: ms,
    ) -> Result<(), SpecialSendError> {
        if destination_device_identifier.0 == BROADCAST_RESERVED_IDENTIFIER {
            return Err(SpecialSendError::BroadcastAddressForbidden);
        }
        let mut current_time = millis::millis();
        let wait_end_time = current_time + timeout;

        while let Some(_) = self.receive() {} // Flush out all messages in the queuee.

        match self._send(PacketMetaData {
            data,
            source_device_identifier: self.my_address.clone(),
            destination_device_identifier: destination_device_identifier.clone(),
            lifetime,
            filter_out_duplication,
            spec_state: SpecState::SendTransaction,
            packet_id: 0,
        }) {
            Ok(_) => (),
            Err(NodeError::SendingQueueIsFull) => {
                return Err(SpecialSendError::TryAgainLater);
            }
        };

        while current_time < wait_end_time {
            let _ = self.update();

            if let Some(answer) = self.receive() {
                if !(answer.spec_state == SpecState::FinishTransaction) {
                    continue;
                }
                if !(answer.source_device_identifier == destination_device_identifier) {
                    continue;
                }
                return Ok(());
            }
            current_time = millis::millis();
        }

        Err(SpecialSendError::Timeout)
    }

    /// Sends the `data` to exact device. or to all devices.
    /// In order to send `data` to all devices, use `BROADCAST_RESERVED_IDENTIFIER`,
    /// otherwise - use identifier of exact device, which is not `BROADCAST_RESERVED_IDENTIFIER`
    /// identifier.
    ///
    /// * `data` - Is the instance of `PacketDataBytes`, which is just type alias of
    /// heapless vector of bytes of special size. This size is configured in the
    /// node/packet/config.rs file, and can be adjusted for case of other data size is needed.
    /// `Note!` That all devices should have same version of protocol flashed, in order to
    /// be able to correctly to communicate with each other.
    ///
    /// * `destination_device_identifier` is instance of Deviceidentifier type,
    /// That type is made for simplicity of reading the code, and to strict possible mess-ups
    /// during the usage of methods. It is made to present device id within the network.
    /// `Note!`, that you can send message to all devices at once.
    /// The reason of that, that in this protocol - there is reserved
    /// `BROADCAST_RESERVED_IDENTIFIER`.
    /// This is the special kind of identifier, made especially to make every node
    /// to be able to recognize this identifier as it's own identifier. In other words, every node
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
        destination_device_identifier: DeviceIdentifier,
        lifetime: LifeTimeType,
        filter_out_duplication: bool,
    ) -> Result<(), NodeError> {
        self._send(PacketMetaData {
            data,
            source_device_identifier: self.my_address.clone(),
            destination_device_identifier,
            lifetime,
            filter_out_duplication,
            spec_state: SpecState::Normal,
            packet_id: 0,
        })
    }

    fn _send(&mut self, packet_meta_data: PacketMetaData) -> Result<(), NodeError> {
        match self.transmitter.send(packet_meta_data) {
            Ok(_) => Ok(()),
            Err(transmitter::TransmitterError::PacketQueueIsFull) => {
                Err(NodeError::SendingQueueIsFull)
            }
        }
    }

    /// Optionally returns `PacketDataBytes` instance with data,
    /// which has been send exactly to this device, or has been
    /// `broadcast`ed trough all the network.
    pub fn receive(&mut self) -> Option<PacketMetaData> {
        self.received_packet_meta_data_queue.pop_front()
    }

    /// Does all necessary internal work of mesh node:
    /// * Receives packets from ether, and manages their further life.
    ///     ** Data of other devices are going to be send back into ether.
    ///     ** Data addressed to current device, will be unpacked and stored.
    pub fn update(&mut self) -> Result<(), NodeUpdateError> {
        if self.timer.is_time_to_speak() {
            self.transmitter.update();
            self.timer.record_speak_time();
        }
        self.receiver.update();

        let packet_to_handle = match self.receiver.receive() {
            Some(packet_to_handle) => packet_to_handle,
            None => return Ok(()),
        };

        let reached_destination_packet = match self.packet_router.route(packet_to_handle) {
            Ok(ok_case) => match ok_case {
                OkCase::Handled => return Ok(()),
                OkCase::Received(packet) => packet,
            },
            Err(err_case) => match err_case {
                ErrCase::TransitQueueIsFull => return Err(NodeUpdateError::TransitQueueIsFull),
                ErrCase::PacketLifetimeEnded => return Ok(()),
            },
        };

        match self
            .received_packet_meta_data_queue
            .push_back(reached_destination_packet)
        {
            Ok(()) => Ok(()),
            Err(_) => Err(NodeUpdateError::ReceivingQueueIsFull),
        }
    }
}
