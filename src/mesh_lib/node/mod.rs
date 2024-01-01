mod constants;
mod packet;
mod receiver;
mod router;
mod timer;
mod transmitter;
mod types;

pub use packet::{AddressType, MULTICAST_RESERVED_IDENTIFIER};
use platform_serial::PlatformSerial;
pub use router::PacketState;
pub use types::NodeString;

pub use packet::{meta_data::PacketMetaData, LifeTimeType, PacketDataBytes};

use self::{
    router::{PacketLifetimeEnded, PacketRouter, RouteResult},
    types::PacketDataQueue,
};

use platform_millis::{ms, PlatformTime};

/// The main structure of the library to use communication
/// in the mesh network. The node works in the manner of listening
/// of ether for some time, which is called `listen_period`, and
/// then sending out packets.
///
/// Also node resends caught packets, that were addressed to other
/// nodes.
///
/// It has next methods:
/// * `new` - Creates new instance of `Node`.
/// * `send` - Sends the `data` to exact device. Call of this method does not provide any
/// response back.
/// * `send_ping_pong` - Sends the `data` to exact device, and the receiving device will
/// be forsed to make answer back. The answer from receiving device
/// may tell if sending was successful.
/// * `send_with_transaction` - Sends the `data` to exact device, and the receiving device will
/// be forsed to make answer back. The answer from receiving device
/// will tell if sending was successful.
/// * `update` - Updates the state of the node. This method should be called in
/// every loop iteration.
pub struct Node {
    transmitter: transmitter::Transmitter,
    receiver: receiver::Receiver,
    my_address: AddressType,
    timer: timer::Timer,
    received_packet_meta_data_queue: PacketDataQueue,
    packet_router: PacketRouter,
}

/// Error that can be returned by `Node` `update` method.
pub struct NodeUpdateError {
    pub is_send_queue_full: bool,
    pub is_transit_queue_full: bool,
}

/// Error that can be returned by `Node` `send` method.
pub enum SendError {
    SendingQueueIsFull,
}

/// Errors, that may occur during the call
/// call of `Node` `send_with_transaction`
/// or `send_ping_pong` method.
pub enum SpecialSendError {
    /// Case when expected response was not received.
    Timeout,

    /// Case, when the destination address is
    /// reserved for multicast address.
    MulticastAddressForbidden,

    /// Case, when the limit of number of
    /// packets to send isreached.
    SendingQueueIsFull,
}

impl From<SendError> for SpecialSendError {
    fn from(value: SendError) -> Self {
        match value {
            SendError::SendingQueueIsFull => SpecialSendError::SendingQueueIsFull,
        }
    }
}

/// User-friendly `Node` configuration structure.
pub struct NodeConfig {
    /// Address of this device. Instance of `AddressType`.
    pub device_address: AddressType,

    /// Instance of `ms` type. The time period in
    /// milliseconds that this device will listen for incoming packets
    /// before speaking back into the ether.
    pub listen_period: ms,
}

impl Node {
    /// Creates new instance of `Node`.
    ///
    /// parameters:
    /// * `config` - Instance of `NodeConfig`.
    pub fn new(config: NodeConfig) -> Node {
        Node {
            transmitter: transmitter::Transmitter::new(),
            receiver: receiver::Receiver::new(),
            my_address: config.device_address.clone(),
            timer: timer::Timer::new(config.listen_period),
            received_packet_meta_data_queue: PacketDataQueue::new(),
            packet_router: PacketRouter::new(config.device_address),
        }
    }

    /// Sends the `data` to exact device, and the receiving device will
    /// be forsed to make answer back. The answer from receiving device
    /// will tell if sending was successful.
    ///
    /// parameters:
    /// * `data` - Is the instance of `PacketDataBytes`, which is just type alias of
    /// heapless vector of bytes of special size. This size is configured in the
    /// node/packet/config.rs file, and can be adjusted for case of other data size is needed.
    /// `Note!` That all devices should have same version of protocol flashed, in order to
    /// be able to correctly to communicate with each other.
    ///
    /// * `destination_device_identifier` is instance of AddressType,
    /// This is made to presend device's address within the network.
    /// `Note!`, that you are prohibited to send message to all devices at once.
    /// Otherwise it will jam the network.
    ///
    /// *`lifetime` - is the instance of `LifeTimeType`. This value configures the count of
    /// how many nodes - the packet will be able to pass. Also this value is needed
    /// to void the ether being jammed by packets, that in theory might be echoed
    /// by the nodes to the infinity...
    ///
    /// *`filter_out_duplication` - Tells if the protocol on the other devices will be ignoring
    /// echoes of this message. It is strongly recommended to use in order to make lower load
    /// onto the network.
    ///
    /// * `timeout` - Is the period of time in milliseconds that
    /// this device will listen for incoming packets
    /// from other devices. In case if no response was caught during that
    /// period of time, the method will return `Err(SpecialSendError::Timeout)`.
    ///
    /// * Call of this method also requires the general types to be passed in.
    /// As the process relies onto timing countings and onto serial stream,
    ///
    /// That parts can be platform dependent, so general trait bound types are made to
    /// be able to use this method in any platform, by just providing platform specific
    /// types.
    pub fn send_ping_pong<TIMER: PlatformTime, SERIAL: PlatformSerial<u8>>(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifier: AddressType,
        lifetime: LifeTimeType,
        filter_out_duplication: bool,
        timeout: ms,
    ) -> Result<(), SpecialSendError> {
        if destination_device_identifier == MULTICAST_RESERVED_IDENTIFIER {
            return Err(SpecialSendError::MulticastAddressForbidden);
        }

        let mut current_time = TIMER::millis();
        let wait_end_time = current_time + timeout;

        while let Some(_) = self.receive() {} // Flush out all messages in the queuee.

        if let Err(any_err) = self._send(PacketMetaData {
            data,
            source_device_identifier: self.my_address.clone(),
            destination_device_identifier: destination_device_identifier.clone(),
            lifetime,
            filter_out_duplication,
            spec_state: PacketState::Ping,
            packet_id: 0,
        }) {
            return Err(any_err.into());
        }

        while current_time < wait_end_time {
            let _ = self.update::<TIMER, SERIAL>();

            if let Some(answer) = self.receive() {
                if !(answer.spec_state == PacketState::Pong) {
                    continue;
                }
                if !(answer.source_device_identifier == destination_device_identifier) {
                    continue;
                }
                return Ok(());
            }
            current_time = TIMER::millis();
        }

        Err(SpecialSendError::Timeout)
    }

    /// Sends the `data` to exact device with the response, that tells if the message
    /// was sent successfully, or sending has failed.
    ///
    /// * `data` - Is the instance of `PacketDataBytes`, which is just type alias of
    /// heapless vector of bytes of special size. This size is configured in the
    /// node/packet/config.rs file, and can be adjusted for case of other data size is needed.
    /// `Note!` That all devices should have same version of protocol flashed, in order to
    /// be able to correctly to communicate with each other.
    ///
    /// * `destination_device_identifier` is instance of AddressType,
    /// That type is made for simplicity of reading the code, and to strict possible mess-ups
    /// during the usage of methods. It is made to present device id within the network.
    /// Multicast trough this method - is prohibited due to keep the network clean from special packets.
    /// In case if you will try to multicast it,
    /// you will get Error with proper reason.
    ///
    /// * `lifetime` - is the instance of `LifeTimeType`. This value configures the count of
    /// how many nodes - the packet will be able to pass. Also this value is needed
    /// to void the ether being jammed by packets, that in theory might be echoed
    /// by the nodes to the infinity...
    /// Each device, once passes transit packet trough it - it reduces packet's lifetime.
    ///
    /// * `filter_out_duplication` - Tells if the protocol on the other devices will be ignoring
    /// echoes of this message. It is strongly recommended to use in order to make lower load
    /// onto the network.
    ///
    /// * `timeout` - Is the period of time in milliseconds that
    /// this device will wait until packet that finishes the transaction - arrives.
    /// In case if no response was caught during that period of time, the method will
    /// return `Err(SpecialSendError::Timeout)`.
    ///
    /// * Call of this method also requires the general types to be passed in.
    /// As the process relies onto timing countings and onto serial stream,
    ///
    /// That parts can be platform dependent, so general trait bound types are made to
    /// be able to use this method in any platform, by just providing platform specific
    /// types.
    pub fn send_with_transaction<TIMER: PlatformTime, SERIAL: PlatformSerial<u8>>(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifier: AddressType,
        lifetime: LifeTimeType,
        filter_out_duplication: bool,
        timeout: ms,
    ) -> Result<(), SpecialSendError> {
        if destination_device_identifier == MULTICAST_RESERVED_IDENTIFIER {
            return Err(SpecialSendError::MulticastAddressForbidden);
        }
        let mut current_time = TIMER::millis();
        let wait_end_time = current_time + timeout;

        while let Some(_) = self.receive() {} // Flush out all messages in the queuee.

        if let Err(any_err) = self._send(PacketMetaData {
            data,
            source_device_identifier: self.my_address.clone(),
            destination_device_identifier: destination_device_identifier.clone(),
            lifetime,
            filter_out_duplication,
            spec_state: PacketState::SendTransaction,
            packet_id: 0,
        }) {
            return Err(any_err.into());
        }

        while current_time < wait_end_time {
            let _ = self.update::<TIMER, SERIAL>();

            if let Some(answer) = self.receive() {
                if !(answer.spec_state == PacketState::FinishTransaction) {
                    continue;
                }
                if !(answer.source_device_identifier == destination_device_identifier) {
                    continue;
                }
                return Ok(());
            }

            current_time = TIMER::millis();
        }

        Err(SpecialSendError::Timeout)
    }

    /// Sends the `data` to exact device. or to all devices.
    /// In order to send `data` to all devices, use `MULTICAST_RESERVED_IDENTIFIER`,
    /// otherwise - use identifier of exact device, which is not `MULTICAST_RESERVED_IDENTIFIER`
    /// identifier.
    ///
    /// * `data` - Is the instance of `PacketDataBytes`, which is just type alias of
    /// heapless vector of bytes of special size. This size is configured in the
    /// node/packet/config.rs file, and can be adjusted for case of other data size is needed.
    /// `Note!` That all devices should have same version of protocol flashed, in order to
    /// be able to correctly to communicate with each other.
    ///
    /// * `destination_device_identifier` is instance of AddressType,
    /// That type is made for simplicity of reading the code, and to strict possible mess-ups
    /// during the usage of methods. It is made to present device id within the network.
    /// `Note!`, that you can send message to all devices at once.
    /// The reason of that, that in this protocol - there is reserved
    /// `MULTICAST_RESERVED_IDENTIFIER`.
    /// This is the special kind of identifier, made especially to make every node
    /// to be able to recognize this identifier as it's own identifier. In other words, every node
    /// will receive the multicast message.
    ///
    /// * `lifetime` - is the instance of `LifeTimeType`. This value configures the count of
    /// how many nodes - the packet will be able to pass. Also this value is needed
    /// to void the ether being jammed by packets, that in theory might be echoed
    /// by the nodes to the infinity...
    /// Each device, once passes transit packet trough it - it reduces packet's lifetime.
    ///
    /// * `filter_out_duplication` - Tells if the protocol on the other devices will be ignoring
    /// echoes of this message. It is strongly recommended to use in order to make lower load
    /// onto the network.
    pub fn send(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifier: AddressType,
        lifetime: LifeTimeType,
        filter_out_duplication: bool,
    ) -> Result<(), SendError> {
        self._send(PacketMetaData {
            data,
            source_device_identifier: self.my_address.clone(),
            destination_device_identifier,
            lifetime,
            filter_out_duplication,
            spec_state: PacketState::Normal,
            packet_id: 0,
        })
    }

    fn _send(&mut self, packet_meta_data: PacketMetaData) -> Result<(), SendError> {
        match self.transmitter.send(packet_meta_data) {
            Ok(_) => Ok(()),
            Err(transmitter::PacketQueueIsFull) => Err(SendError::SendingQueueIsFull),
        }
    }

    /// Optionally returns `PacketDataBytes` instance with data,
    /// which has been send exactly to this device, or has been
    /// `multicast`ed trough all the network.
    pub fn receive(&mut self) -> Option<PacketMetaData> {
        self.received_packet_meta_data_queue.pop_front()
    }

    /// Does all necessary internal work of mesh node:
    /// * Receives packets from ether, and manages their further life.
    ///     ** Data that is addressed to other devices are going to be send back into ether.
    ///     ** Data addressed to current device, will be unpacked and stored.
    ///
    /// * Call of this method also requires the general types to be passed in.
    /// As the process relies onto timing countings and onto serial stream,
    ///
    /// That parts can be platform dependent, so general trait bound types are made to
    /// be able to use this method in any platform, by just providing platform specific
    /// types with all required traits implemented.
    pub fn update<TIMER: PlatformTime, SERIAL: PlatformSerial<u8>>(
        &mut self,
    ) -> Result<(), NodeUpdateError> {
        let current_time = TIMER::millis();

        if self.timer.is_time_to_speak(current_time) {
            self.transmitter.update::<SERIAL>();
            self.timer.record_speak_time(current_time);
        }
        self.receiver.update::<SERIAL>(current_time);

        let packet_to_handle = match self.receiver.receive(current_time) {
            Some(packet_to_handle) => packet_to_handle,
            None => return Ok(()),
        };

        let (received_packet, transit_packet): (Option<PacketMetaData>, Option<PacketMetaData>) =
            match self.packet_router.route(packet_to_handle) {
                Ok(ok_case) => match ok_case {
                    RouteResult::Received(packet) => (Some(packet), None),
                    RouteResult::Transit(transit) => (None, Some(transit)),
                    RouteResult::ReceivedAndTransit { received, transit } => {
                        (Some(received), Some(transit))
                    }
                },
                Err(PacketLifetimeEnded) => (None, None),
            };

        let (mut is_send_queue_full, mut is_transit_queue_full): (bool, bool) = (false, false);

        if let Some(received_packet) = received_packet {
            match self
                .received_packet_meta_data_queue
                .push_back(received_packet)
            {
                Ok(()) => (),
                Err(_) => {
                    is_send_queue_full = true;
                }
            }
        }

        if let Some(transit_packet) = transit_packet {
            match self.transmitter.send_transit(transit_packet) {
                Ok(_) => (),
                Err(transmitter::PacketTransitQueueIsFull) => {
                    is_transit_queue_full = true;
                }
            }
        }

        if (!is_send_queue_full) && (!is_transit_queue_full) {
            Ok(())
        } else {
            Err(NodeUpdateError {
                is_send_queue_full,
                is_transit_queue_full,
            })
        }
    }
}
