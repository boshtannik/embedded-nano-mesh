mod constants;
mod packet;
mod receiver;
mod router;
mod timer;
mod transmitter;
mod types;

pub use packet::{
    ExactAddressType, GeneralAddressType, IdType, LifeTimeType, Packet, PacketDataBytes,
};

pub use router::PacketState;
use types::PacketQueue;
pub use types::{ms, NodeString};

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

impl core::fmt::Debug for SendError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SendError::SendingQueueIsFull => write!(f, "SendingQueueIsFull"),
        }
    }
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

impl core::fmt::Debug for SpecialSendError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SpecialSendError::Timeout => write!(f, "Timeout"),
            SpecialSendError::SendingQueueIsFull => write!(f, "SendingQueueIsFull"),
        }
    }
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
    /// New Method
    /// To initialize a `Node`, you need to provide `NodeConfig` with values:
    /// - `ExactAddressType`: Sets the device's identification address in the network. Multiple deivces can share same address in the same network.
    /// - `listen_period`: Sets period in milliseconds that determines how long the device will wait before transmitting packet to the network. It prevents network congestion.

    /// `main.rs`:
    /// ```
    /// let mut mesh_node = Node::new(NodeConfig {
    ///     device_address: ExactAddressType::new(1).unwrap(),
    ///     listen_period: 150 as ms,
    /// });
    /// ```
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

    /// Send Ping-Pong Method
    /// Sends a message with a "ping" flag to the destination node and
    /// waits for the same message with a "pong" flag. Return value tells that the end device have received
    /// the message at least once or returns an error if the ping-pong exchange fails.
    /// The following arguments are required:
    ///
    /// `Ping-Pong time diagram`:
    ///            +----------+              +----------+
    ///            |  Sender  |              | Receiver |
    ///            +--------- +              +----------+
    ///                 |                         |
    /// Ping-pong start |   --------Ping------->  |   <-- Receiver has received the message
    ///                 |                         |
    /// Ping-pong finish|   <-------Pong--------  |
    ///                 |                         |
    ///
    ///`main.rs`:
    ///```
    ///let _ = mesh_node.send_ping_pong(
    ///    message.into_bytes(),               // Content.
    ///    ExactAddressType::new(2).unwrap(),  // Send to device with address 2.
    ///    10 as LifeTimeType,                 // Let message travel 10 devices before being destroyed.
    ///    1000 as ms,                         // Set timeout to 1000 ms.
    ///    || {
    ///        Instant::now()
    ///            .duration_since(program_start_time)
    ///            .as_millis() as ms
    ///    },                                  // Closure providing current time in milliseconds.
    ///    &mut serial,                        // IO interface.
    ///);
    ///```
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
    /// * `millis_provider` - Is the closure that returns current time in milliseconds,
    ///
    /// * `interface_driver` - Is the instance of `embedded_serial::MutNonBlockingRx`
    ///                        and `MutBlockingTx` traits.
    ///                        In other words the driver which will be used to
    ///                        read and write from the interface.
    pub fn send_ping_pong<I, M>(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifier: ExactAddressType,
        lifetime: LifeTimeType,
        timeout: ms,
        millis_provider: M,
        interface_driver: &mut I,
    ) -> Result<(), SpecialSendError>
    where
        I: embedded_io::ReadReady + embedded_io::Read + embedded_io::Write,
        M: Fn() -> ms,
    {
        self._special_send(
            data,
            destination_device_identifier,
            PacketState::Ping,
            PacketState::Pong,
            lifetime,
            timeout,
            millis_provider,
            interface_driver,
        )
    }

    /// Send with Transaction Method
    /// Sends a message and handles all further work to
    /// ensure the target device have received it only once.
    /// Method returns an error if the transaction failed.
    ///
    /// `Transaction time diagram`:
    /// ```
    ///                       +----------+              +----------+
    ///                       |  Sender  |              | Receiver |
    ///                       +--------- +              +----------+
    ///                            |                         |
    ///     *Transaction start     | ---SendTransaction--->  |
    ///                            |                         |
    ///                    /       | <--AcceptTransaction--  |
    /// (increment packet id by 1) |                         |
    ///                    \       | ---InitTransaction--->  |    <--- Receiver has received the message
    ///                            |                         |
    ///     *Transaction finish    | <--FinishTransaction--  |
    ///                            |                         |
    /// ```
    ///
    /// `main.rs`:
    /// ```
    /// match mesh_node.send_with_transaction(
    ///     message.into_bytes(),               // Content.
    ///     ExactAddressType::new(2).unwrap(),  // Send to device with address 2.
    ///     10 as LifeTimeType,                 // Let message travel 10 devices before being destroyed.
    ///     2000 as ms,                         // Wait 2 seconds for response.
    ///     || {
    ///         Instant::now()
    ///             .duration_since(program_start_time)
    ///             .as_millis() as ms
    ///     },                                  // Closure providing current time in milliseconds.
    ///     &mut serial,                        // IO interface.
    /// );
    /// ```
    /// parameters:
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
    /// * `millis_provider` - Is the closure that returns current time in milliseconds,
    ///
    /// * `interface_driver` - Is the instance of `embedded_serial::MutNonBlockingRx`
    ///                        and `MutBlockingTx` traits.
    ///                        In other words the driver which will be used to
    ///                        read and write from the interface.
    pub fn send_with_transaction<I, M>(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifier: ExactAddressType,
        lifetime: LifeTimeType,
        timeout: ms,
        millis_provider: M,
        interface_driver: &mut I,
    ) -> Result<(), SpecialSendError>
    where
        I: embedded_io::ReadReady + embedded_io::Read + embedded_io::Write,
        M: Fn() -> ms,
    {
        self._special_send(
            data,
            destination_device_identifier,
            PacketState::SendTransaction,
            PacketState::FinishTransaction,
            lifetime,
            timeout,
            millis_provider,
            interface_driver,
        )
    }

    fn _special_send<I, M>(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifier: ExactAddressType,
        request_state: PacketState,
        expected_response_state: PacketState,
        lifetime: LifeTimeType,
        timeout: ms,
        millis_provider: M,
        interface_driver: &mut I,
    ) -> Result<(), SpecialSendError>
    where
        I: embedded_io::ReadReady + embedded_io::Read + embedded_io::Write,
        M: Fn() -> ms,
    {
        let mut current_time = millis_provider();
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
                // Following the transaction time diagram - it is expected the packet to
                // have it's id increased three times.
                PacketState::SendTransaction => expected_response_packet_id + 1,
                PacketState::Ping => expected_response_packet_id,
                _ => expected_response_packet_id,
            },
        };

        while current_time < wait_end_time {
            let _ = self.update(interface_driver, current_time);

            if let Some(answer) = self.receive() {
                if !(answer.source_device_identifier == destination_device_identifier.into()) {
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

            current_time = millis_provider();
        }

        Err(SpecialSendError::Timeout)
    }

    /// Send to exact Method
    /// Sends the message to device with exact address in the network.
    /// The `send_to_exact` method requires the following arguments:
    ///
    /// `main.rs`:
    /// ```
    /// let _ = match mesh_node.send_to_exact(
    ///     message.into_bytes(),              // Content.
    ///     ExactAddressType::new(2).unwrap(), // Send to device with address 2.
    ///     10 as LifeTimeType, // Let message travel 10 devices before being destroyed.
    ///     true, // filter_out_duplication
    /// );
    /// ```
    ///
    /// * `data` - Is the instance of `PacketDataBytes`, which is just type alias of
    /// heapless vector of bytes of special size. This size is configured in the
    /// node/packet/config.rs file.
    /// `Note!` That all devices should have same version of protocol flashed, in order to
    /// have best compatibility with each other.
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
    /// * `filter_out_duplication` - Tells if the other devices shall ignore
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
            0, // Anyway it will be set later in the trasmitter.
            lifetime,
            PacketState::Normal,
            filter_out_duplication,
            data,
        )) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    /// Broadcast Method
    /// Shares the message to all nodes in the network.
    /// Distance of sharing is set by `lifetime` parameter.
    /// It sends packet with destination address set as
    /// `GeneralAddressType::BROADCAST`. Every device will treats `GeneralAddressType::Broadcast`
    /// as it's own address, so they keep the message as received and transits copy of that message further.
    /// `main.rs`:
    /// ```
    /// let _ = mesh_node.broadcast(
    ///     message.into_bytes(), // data.
    ///     10 as LifeTimeType,   // lifetime.
    /// );
    /// ```
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

    /// Receive Method
    /// Optionally returns `PacketDataBytes` instance with data,
    /// which has been send exactly to this device, or has been
    /// `broadcast`ed trough all the network.
    ///
    /// You can tell which type the packet is by matching `special_state` field of returned `Packet` instance.
    /// Field contains value of `PacketState` enum.
    ///
    /// `main.rs`:
    /// ```
    /// match mesh_node.receive() {
    ///     Some(packet) => ...,
    ///     Node => ....,
    /// }
    /// ```

    pub fn receive(&mut self) -> Option<Packet> {
        self.received_packet_queue.pop_front()
    }

    /// Update Method
    /// The most important method.
    /// During call of `update` method - it does all internal work:
    /// - routes packets trough the network
    /// - transits packets that were sent to other devices
    /// - handles `lifetime` of packets
    /// - handles special packets like `ping` and `pong`, or any kind of transaction one.
    /// - saves received packets that will be available trough `receive` method.
    /// - sends packets, that are in the `send` queue.
    ///
    /// As the protocol relies on physical device - it is crucial to provide
    /// driver for communication interface.
    /// Also node shall know if it's the time to broadcast into the ether or not,
    /// so for that purpose the closure that counts milliseconds since program start
    /// is required.
    ///
    /// Methods: `send_ping_pong`, `send_with_transaction` also relies on `millis_provider` closure and `interface_driver`.
    /// With out call this method in a loop - the node will stop working.
    ///
    ///`main.rs`:
    ///```
    /// loop {
    ///    let current_time = Instant::now()
    ///        .duration_since(program_start_time)
    ///        .as_millis() as ms;
    ///
    ///    let _ = mesh_node.update(&mut serial, current_time);
    /// }
    ///```

    /// Does all necessary internal work of mesh node:
    /// * Receives packets from ether, and manages their further life.
    ///     ** Data that is addressed to other devices are going to be send back into ether.
    ///     ** Data addressed to current device, will be unpacked and stored.
    ///
    /// * Call of this method also requires the general types to be passed in.
    /// As the process relies onto timing countings and onto serial stream,
    ///
    /// parameters:
    /// * `interface_driver` - is instance of `MutNonBlockingRx` and `MutBlockingTx`
    /// traits.
    ///
    /// * `current_time` - Is a closure which returns current time in milliseconds
    /// since the start of the program.
    pub fn update<I>(
        &mut self,
        interface_driver: &mut I,
        current_time: ms,
    ) -> Result<(), NodeUpdateError>
    where
        I: embedded_io::ReadReady + embedded_io::Read + embedded_io::Write,
    {
        if self.timer.is_time_to_speak(current_time) {
            self.transmitter.update(interface_driver);
            self.timer.record_speak_time(current_time);
        }
        self.receiver.update(current_time, interface_driver);

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
