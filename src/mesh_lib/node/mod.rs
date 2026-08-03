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
    /// Whether the received packet queue is full, meaning that a packet
    /// addressed to this node could not be stored.
    pub is_receive_queue_full: bool,
    /// Whether the transit queue is full, meaning that a packet addressed
    /// to another node could not be scheduled for forwarding.
    pub is_transit_queue_full: bool,
}

/// Error that can be returned by `Node` `send` method or `broadcast` method.
pub enum SendError {
    /// The sending queue is full. Retry sending later.
    SendingQueueIsFull,
}

impl core::fmt::Debug for SendError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SendError::SendingQueueIsFull => write!(f, "SendingQueueIsFull"),
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
    /// ```ignore
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

    /// Send to exact Method
    /// Sends the message to device with exact address in the network.
    /// The `send_to_exact` method requires the following arguments:
    ///
    /// `main.rs`:
    /// ```ignore
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
    /// ```ignore
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
    /// `main.rs`:
    /// ```ignore
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
    /// - saves received packets that will be available trough `receive` method.
    /// - sends packets, that are in the `send` queue.
    ///
    /// As the protocol relies on physical device - it is crucial to provide
    /// driver for communication interface.
    /// Also node shall know if it's the time to broadcast into the ether or not,
    /// so for that purpose the closure that counts milliseconds since program start
    /// is required.
    ///
    /// With out call this method in a loop - the node will stop working.
    ///
    ///`main.rs`:
    ///```ignore
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
