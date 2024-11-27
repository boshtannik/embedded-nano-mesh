mod constants;
mod packet;
mod receiver;
mod router;
mod timer;
mod transmitter;
mod types;

use packet::{traits::GettersSetters, Packet};
pub use packet::{ExactAddressType, GeneralAddressType, IdType, LifeTimeType, PacketDataBytes};

pub use platform_millis::{ms, PlatformMillis};
pub use platform_serial::PlatformSerial;

pub use router::PacketState;
pub use types::NodeString;
use types::PacketQueue;

use self::router::{RouteError, RouteResult, Router};

/// The main and only structure of the library that brings API for
/// communication trough the mesh network.
/// It works in the manner of listening of ether for
/// specified period of time, which is called `listen_period`,
/// and then sending out packets out of queues between those periods.
///
/// Also node resends caught packets, that were addressed to other
/// nodes.
///
/// It has next methods:
/// * `new` -                   Creates new instance of `Node`.
/// * `send_to_exact` -         Sends the `data` to exact device. Call of this method does not provide any
///                             response back.
/// * `broadcast` -             Sends the `data` to all devices. Call of this method does not provide any
///                             response back.
/// * `send_ping_pong` -        Sends the `data` to exact device, and the receiving device will
///                             be forsed to make answer back. The answer from receiving device
///                             may tell if sending was successful.
/// * `send_with_transaction` - Sends the `data` to exact device, and the receiving device will
///                             be forsed to make answer back. The answer from receiving device
///                             will tell if sending was successful.
/// * `update` -                Updates the state of the node. This method should be called in
///                             every loop iteration.
pub struct Node {
    transmitter: transmitter::Transmitter,
    receiver: receiver::Receiver,
    my_address: ExactAddressType,
    timer: timer::Timer,
    received_packet_queue: PacketQueue,
    router: Router,
}

/// Error that can be returned by `Node` `update` method.
pub struct NodeUpdateError {
    pub is_receive_queue_full: bool,
    pub is_transit_queue_full: bool,
}

/// Error that can be returned by `Node` `send` method or `broadcast` method.
pub enum SendError {
    SendingQueueIsFull,
}

/// Errors, that may occur during the call
/// of `Node` `send_with_transaction` or `send_ping_pong` method.
pub enum SpecialSendError {
    /// Case when expected response was not received.
    Timeout,

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
    /// Address of configurable device. Instance of `ExactAddressType`.
    pub device_address: ExactAddressType,

    /// Instance of `ms` type. The time period in
    /// milliseconds that configured device will listen for incoming packets
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
            received_packet_queue: PacketQueue::new(),
            router: Router::new(config.device_address.into()),
        }
    }

    /// Sends the `data` to exact device with `ping` flag set, and the receiving device will
    /// be forsed to make answer back with 'pong' flag set. The answer from receiving device
    /// will tell if sending was successful.
    ///
    /// parameters:
    /// * `data` - Is the instance of `PacketDataBytes`, which is just type alias of
    /// heapless vector of bytes of special size. This size is configured in the
    /// node/packet/config.rs file, and can be adjusted for case of other data size is needed.
    /// `Note!` That all devices should have same version of protocol flashed, in order to
    /// be able to correctly to communicate with each other.
    ///
    /// * `destination_device_identifier` is instance of ExactDeviceAddressType,
    /// This is made to presend device's address within the network.
    ///
    /// *`lifetime` - is the instance of `LifeTimeType`. This value configures the count of
    /// how many nodes - the packet will be able to pass. Also this value is needed
    /// to void the ether being jammed by packets, that in theory might be echoed
    /// by the nodes to the infinity...
    ///
    /// * `timeout` - Is the period of time in milliseconds that
    /// this device will listen for response. In case if no response was caught during that
    /// period of time, the method will return `Err(SpecialSendError::Timeout)`.
    ///
    /// * Call of this method also requires the general types to be passed in.
    /// As the process relies onto timing countings and onto serial stream,
    ///
    /// That parts can be platform dependent, so general trait bound types are made to
    /// be able to use this method in any platform, by just providing platform specific
    /// types.
    pub fn send_ping_pong<TIMER: PlatformMillis, SERIAL: PlatformSerial<u8>>(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifier: ExactAddressType,
        lifetime: LifeTimeType,
        timeout: ms,
    ) -> Result<(), SpecialSendError> {
        self._special_send::<TIMER, SERIAL>(
            data,
            destination_device_identifier,
            PacketState::Ping,
            PacketState::Pong,
            lifetime,
            timeout,
        )
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
    /// * `destination_device_identifier` is instance of `ExactDeviceAddressType`,
    /// That type is made for simplicity of reading the code, and to strict possible mess-ups
    /// during the usage of methods. It is made to present device id within the network.
    ///
    /// * `lifetime` - is the instance of `LifeTimeType`. This value configures the count of
    /// how many nodes - the packet will be able to pass. Also this value is needed
    /// to void the ether being jammed by packets, that in theory might be echoed
    /// by the nodes to the infinity...
    /// Each device, once passes transit packet trough it - it reduces packet's lifetime.
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
    pub fn send_with_transaction<TIMER: PlatformMillis, SERIAL: PlatformSerial<u8>>(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifier: ExactAddressType,
        lifetime: LifeTimeType,
        timeout: ms,
    ) -> Result<(), SpecialSendError> {
        self._special_send::<TIMER, SERIAL>(
            data,
            destination_device_identifier,
            PacketState::SendTransaction,
            PacketState::FinishTransaction,
            lifetime,
            timeout,
        )
    }

    fn _special_send<TIMER: PlatformMillis, SERIAL: PlatformSerial<u8>>(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifier: ExactAddressType,
        request_state: PacketState,
        expected_response_state: PacketState,
        lifetime: LifeTimeType,
        timeout: ms,
    ) -> Result<(), SpecialSendError> {
        let mut current_time = TIMER::millis();
        let wait_end_time = current_time + timeout;

        while let Some(_) = self.receive() {} // Flush out all messages in the queuee.

        let expected_response_packet_id = match self._send(Packet::new(
            self.my_address.into(),
            destination_device_identifier.into(),
            0,
            lifetime,
            request_state.clone(),
            true,
            data,
        )) {
            Err(any_err) => return Err(any_err.into()),
            Ok(expected_response_packet_id) => match request_state {
                // It is needed to wait for response packet with specific packet id.
                PacketState::SendTransaction => expected_response_packet_id + 1,
                PacketState::Ping => expected_response_packet_id,
                _ => expected_response_packet_id,
            },
        };

        while current_time < wait_end_time {
            let _ = self.update::<TIMER, SERIAL>();

            if let Some(answer) = self.receive() {
                if !(answer.get_source_device_identifier() == destination_device_identifier.into())
                {
                    continue;
                }
                if !(answer.get_spec_state() == expected_response_state) {
                    continue;
                }
                if !(answer.get_id() == expected_response_packet_id) {
                    continue;
                }
                return Ok(());
            }

            current_time = TIMER::millis();
        }

        Err(SpecialSendError::Timeout)
    }

    /// Sends the `data` to exact device.
    /// In order to send `data` to exact device`, use
    /// ExactAddressType::new(...).unwrap()` identifier
    ///
    /// * `data` - Is the instance of `PacketDataBytes`, which is just type alias of
    /// heapless vector of bytes of special size. This size is configured in the
    /// node/packet/config.rs file.
    /// `Note!` That all devices should have same version of protocol flashed, in order to
    /// be able to correctly to communicate with each other.
    ///
    /// * `destination_device_identifier` is instance of `ExactAddressType`,
    /// That type is made to limit possible mess-ups during the usage of method.
    ///
    /// * `lifetime` - is the instance of `LifeTimeType`. This value configures the count of
    /// how many nodes - the packet will be able to pass. Also this value is provided
    /// to void the ether being jammed by packets, that in theory might be echoed
    /// by other nodes to the infinity...
    /// Each device, once passes transit packet trough it - it reduces packet's lifetime.
    ///
    /// * `filter_out_duplication` - Tells if the other devices will be ignoring
    /// echoes of this message. It is strongly recommended to use in order to make lower load
    /// onto the network.
    pub fn send_to_exact(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifier: ExactAddressType,
        lifetime: LifeTimeType,
        filter_out_duplication: bool,
    ) -> Result<(), SendError> {
        match self._send(Packet::new(
            self.my_address.into(),
            destination_device_identifier.into(),
            0,
            lifetime,
            PacketState::Normal,
            filter_out_duplication,
            data,
        )) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Sends the `data` to all devices.
    ///
    /// * `data` - Is the instance of `PacketDataBytes`, which is just type alias of
    /// heapless vector of bytes of special size. This size is configured in the
    /// node/packet/config.rs file.
    /// `Note!` That all devices should have same version of protocol flashed, in order to
    /// be able to correctly to communicate with each other.
    ///
    /// * `lifetime` - is the instance of `LifeTimeType`. This value configures the count of
    /// how many nodes - the packet will be able to pass. Also this value is provided
    /// to void the ether being jammed by packets, that in theory might be echoed
    /// by other nodes to the infinity...
    /// Each device, once passes transit packet trough it - it reduces packet's lifetime.
    pub fn broadcast(
        &mut self,
        data: PacketDataBytes,
        lifetime: LifeTimeType,
    ) -> Result<(), SendError> {
        match self._send(Packet::new(
            self.my_address.into(),
            GeneralAddressType::Broadcast.into(),
            0,
            lifetime,
            PacketState::Normal,
            true,
            data,
        )) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn _send(&mut self, packet: Packet) -> Result<IdType, SendError> {
        match self.transmitter.send(packet) {
            Ok(generated_packet_id) => Ok(generated_packet_id),
            Err(transmitter::PacketQueueIsFull) => Err(SendError::SendingQueueIsFull),
        }
    }

    /// Optionally returns `PacketDataBytes` instance with data,
    /// which has been send exactly to this device, or has been
    /// `broadcast`ed trough all the network.
    ///
    /// Returns `None` if no data is available.
    pub fn receive(&mut self) -> Option<Packet> {
        self.received_packet_queue.pop_front()
    }

    /// Does all necessary internal work of mesh node:
    /// * Receives packets from ether, and manages their further life.
    ///     ** Data that is addressed to other devices are going to be send back into ether.
    ///     ** Data addressed to current device, will be unpacked and stored.
    ///
    /// * Call of this method also requires the general types to be passed in.
    /// As the process relies onto timing countings and onto serial stream,
    ///
    /// That parts are platform dependent, so general trait bound types are made to
    /// be able to use this method in any platform, by just providing platform specific
    /// types with all required traits implemented.
    pub fn update<TIMER: PlatformMillis, SERIAL: PlatformSerial<u8>>(
        &mut self,
    ) -> Result<(), NodeUpdateError> {
        let current_time = TIMER::millis();

        if self.timer.is_time_to_speak(current_time) {
            self.transmitter.update::<SERIAL>();
            self.timer.record_speak_time(current_time);
        }
        self.receiver.update::<SERIAL>(current_time);

        let packet_to_route = match self.receiver.receive(current_time) {
            Some(packet_to_handle) => packet_to_handle,
            None => return Ok(()),
        };

        let (received_packet, transit_packet) = match self.router.route(packet_to_route) {
            Ok(ok_case) => match ok_case {
                RouteResult::ReceivedOnly(packet) => (Some(packet), None),
                RouteResult::TransitOnly(transit) => (None, Some(transit)),
                RouteResult::ReceivedAndTransit { received, transit } => {
                    (Some(received), Some(transit))
                }
            },
            Err(RouteError::PacketLifetimeEnded) => (None, None),
            Err(RouteError::RespondToBroadcastAddressError) => (None, None),
        };

        let (mut is_receive_queue_full, mut is_transit_queue_full): (bool, bool) = (false, false);

        if let Some(received_packet) = received_packet {
            match self.received_packet_queue.push_back(received_packet) {
                Ok(()) => (),
                Err(_) => {
                    is_receive_queue_full = true;
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

        if is_receive_queue_full || is_transit_queue_full {
            return Err(NodeUpdateError {
                is_receive_queue_full,
                is_transit_queue_full,
            });
        } else {
            Ok(())
        }
    }
}
