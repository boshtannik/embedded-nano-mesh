# Mesh Network Protocol for embedded devices
Pure version of embedded-nano-mesh protocol.
This version has no ping_pong and transaction sending methods and
their logic implementation.

This is the mesh network protocol. It allows to turn almost any
kind of MCU device + Radio module device into a mesh node. It is designed
to be lightweight, easy to use and portable to many plaftorms. The protocol
may use variety of radio modules.

The network extends and heals itself automatically by communicating
with other nodes, which have same protocol version installed. Most
versions of this protocol are compatible, but for the best performance
it is recommended to use same and the latest version.

The protocol has been tested with radio
modules JDY-40 during the development, and potentially can
use other radio modules, which might be or your choice:
- JDY-41
- SV-610
- HC-11
- HC-12
- LC12S
- GT-38
- LoRa modules

MCU - stands for Microcontroller Computer Unit. (Arduino, Raspberry Pi, PC, etc.)

## Mesh node architecture:
```
 (Library code runs here which turns MCU into a mesh node)
        |                                          
        |                                          
        V           
+----------------+                 +-----------------+
|                |  (IO Interface) |                 |
|      MCU       |<--------------->|   Radio module  |
|                |                 |                 |
+----------------+                 +-----------------+
```

## Goal:
The goal of this project is to provide ability to build mesh
network with low memory, cheap and easily accessible microcontollers + radio modules
or even personal computer + radio modules to participate the same network.
This protocol can be used for:
- Home automation
- Remote control
- Remote monitoring (telemetry)
- Decentralized messaging
- etc.

## Help to manage the development by filling that form:
https://docs.google.com/forms/d/e/1FAIpQLSeB8In06PumPDxdIu-dGr79nFVyfd1nxgKnxvUXoItIFMfO7Q/viewform?usp=header

## Working principle:
### The way, how the protocol spreads the data:
The protocol routes the packets in the most dumb way.
It spereads the packet in the way, similar to the spread of the atenuating
wave in the pool. It means, that all near devices, that can catch the packet - cathes it.
Then the device's router - determines if the packet is reached it's
destination or the packet has to be transitted further into the network.

Range of operation is regulated by the `lifetime` during the sending.

While packet is being travel trough the network - it's Lifetime is decreased by intermediate devices.
Once `lifetime` value is reached zero during routing - the packet gets destroyed
by the exact device which currently transits it.

The packets, that were just sent by user by `send_to_exact`, `broadcast` method in the device,
which performs the operation - that packets bypasses routing and are sent directly into
sending queue, and then into the ether. It means that lifetime of the packet is not decreased by the router
of the device. So the message can be sent even with `lifetime` set to `0`, anyway it will be transmitted
in the ether for the first time.
Sending of packets from the queues happends during call of `update` method.

Node have 2 internal queues, queues are not exposed to user:
- `send` - for packets that node holds to be sent.
- `transit` - for packets, that node holds to be transitted from other devices.
Sizes of those queues are configurable. And their both configuration of size
is made by `PACKET_QUEUE_SIZE` constant in `./src/mesh_lib/node/constants.rs`.

### How the protocol avoid packet duplication:
Protocol provides mechanism to prevent the network being jammed by
packet duplication.

During send of the packet using `send_to_exact` method - you can set `filter_out_duplication` parameter
to `true` which prevents network from being jammed by duplicated packets.

`filter_out_duplication` leads protocol to spread one exact packet trough the network only once by
setting `ignore_duplication` flag of the packet.

`filter_out_duplication` works in the next way:
1 - Node sets `ignore_duplication` flag to the packet flags.
2 - Once intermediate node receives the packet with `ignore_duplication` flag set to `true`,
2.1 it remembers the `sender_device_identifier` of the packet and `id` of the packet for the
specified `RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD` period of time. if the packet with same
`sender_device_identifier` and with same `id` is received again by that same node - node
ignores packet for that specified period of time.
 
`RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD` period of time. This period is configurable in `./src/mesh_lib/node/constants.rs`.

## Status:
* The version is: 2.1.3:
  Every planned functionality is working. It is:
  - Send data.
  - Receive data.
  - Send data with ignorance of duplicated packets.
  - Send data with limited of number of hops.
  - Broadcast data to all nodes.
  - Message transition by the intermediate nodes.
* Fully backward compatible with no ping_pong or transition functionality.

## Cross-platform compatibility
Protocol currently natively runs on:
- Arduino nano
- Linux (Raspberry PI, Desktop)

Not tested yet on:
- Windows
- Mac
- STM32
- ESP8266
- Raspberry PI pico

## Porting to other platforms
Initially it was designed to be able to run on Arduino nano - it can run
on huge variety of other microcontollers or personal computers.
Protocol is using `embedded-io` trait to communicate with radio modules.
To run the protocol on new platform - the implementation of `embedded-io`
must be provided.

## Issues and discussions:
Contacs are:
- Telegram channel: [https://t.me/embedded_nano_mesh](https://t.me/embedded_nano_mesh)
- Github: [https://github.com/boshtannik](https://github.com/boshtannik)
This will help this project grow.


## Usage:
1 - Include library.
`Cargo.toml`:
```
embedded-nano-mesh = "2.1.1"
```

2 - Include implementation of `embedded-io` or implement it for
your platform.
`Cargo.toml`:
```
embedded-nano-mesh-linux-io = "0.0.1" # For linux
# embedded-nano-mesh-arduino-nano-io = { git = "https://github.com/boshtannik/embedded-nano-mesh-arduino-nano-io.git" } # For arduino
```

3 - initialize and use your implementation of `embedded-io` in your code:
`src/main.rs`:
```
    let mut interface = LinuxIO::new( ... );
    let mut mesh_node = ...;

    match mesh_node.send_to_exact(
        message.into_bytes(),               // Content.
        ExactAddressType::new(2).unwrap(),  // Send to device with address 2.
        10 as LifeTimeType,                 // Let message travel 10 devices before being destroyed.
        true,                               // Filter out duplicated messages.
    ) {
        Ok(()) => {
            println!("Message sent")
        }
        Err(SendError::SendingQueueIsFull) => {
            println!("SendingQueueIsFull")
        }
    }

    loop {
        let _ = mesh_node.update(&mut interface, current_time);
    }
```
During sending of message you can regulate the distance that the packet will be able to
make - by setting the `lifetime` parameter.
For example:
- setting `lifetime` to 1 will limit the message's reach to the nearest devices in the network.
- setting `lifetime` to 10 will make the packet able to pass 10 nodes before being destroyed.

Full examples are available below.

## Arduino nano examples.
Usage examples can be found here:
- [arduino-nano](https://github.com/boshtannik/embedded-nano-mesh-arduino-nano-example)

Sometimes code binary might not fit onto your arduino board memory, in order to
reduce the size of final binary - it is recommended to compile it with
--release flag - it increases optimisation level tlat leads to smaller binary.

## Linux examples.
Usage examples can be found here:
- [linux](https://github.com/boshtannik/embedded-nano-mesh-linux-example)

## API description
All API of the library is provided by the `Node` structure. It offers interface for
actions for `send_to_exact`, `broadcast`, `receive`.

The `Node` must be constantly updated by call its `update` method.
During call of `update` method - it does all internal work:
- routes packets trough the network
- transits packets that were sent to other devices
- handles `lifetime` of packets
- handles special packets like `ping` and `pong`, or any kind of transaction one.
- saves received packets that will be available trough `receive` method.
- sends packets, that are in the `send` queue.

As the protocol relies on physical environment - it is crucial to provide
ability to the library to rely on time counting and on communication interface.
Time calculation is provided by `millis_provider` closure, and `interface_driver`
is described above by `embedded-io` traits.

Method `update` relies on `millis_provider` closure and `interface_driver`.
`interface_driver` - is used for communication with radio modules.
`millis_provider` - is used for time counting.

### New Method
To initialize a `Node`, you need to provide `NodeConfig` with values:
- `ExactAddressType`: Sets the device's identification address in the network. Multiple deivces can share same address in the same network.
- `listen_period`: Sets period in milliseconds that determines how long the device will wait before transmitting packet to the network. It prevents network congestion.

`main.rs`:
```
let mut mesh_node = Node::new(NodeConfig {
    device_address: ExactAddressType::new(1).unwrap(),
    listen_period: 150 as ms,
});
```

### Broadcast Method
Shares the message to all nodes in the network.
Distance of sharing is set by `lifetime` parameter.
It sends packet with destination address set as
`GeneralAddressType::BROADCAST`. Every device will treats `GeneralAddressType::Broadcast`
as it's own address, so they keep the message as received and transits copy of that message further.
`main.rs`:
```
let _ = mesh_node.broadcast(
    message.into_bytes(), // Content.
    10 as LifeTimeType,   // Let message travel 10 devices before being destroyed.
);
```

### Send to exact Method
Sends the message to device with exact address in the network.
The `send_to_exact` method requires the following arguments:

- `data`: A `PacketDataBytes` instance to hold the message bytes.
- `destination_device_identifier`: A `ExactAddressType` instance indicating exact target device.
- `lifetime`: A `LifeTimeType` instance to control for how far the message can travel.
!The term `echoed message` refers to a duplicated message that has
been re-transmitted into the ether by an intermediate device.
- `filter_out_duplication`: A boolean flag to filter out echoed messages from the network.

`main.rs`:
```
let _ = match mesh_node.send_to_exact(
    message.into_bytes(),              // Content.
    ExactAddressType::new(2).unwrap(), // Send to device with address 2.
    10 as LifeTimeType, // Let message travel 10 devices before being destroyed.
    true,
);
```

### Receive Method
Optionally returns received `Packet` instance in case
if that packet was previously received by this exact device. It does not matter if that data
was sent via `broadcast`, `send_to_exact`, `ping_pong` or `send_with_transaction` method because
anyway it was sent to that exact device.
You can tell which type the packet is by matching `special_state` field of returned `Packet` instance.
Field contains value of `PacketState` enum.

`main.rs`:
```
match mesh_node.receive() {
    Some(packet) => ...,
    Node => ....,
}
```

### Update Method
Performs all necessary internal operation of the `Node`.
It must be called in a loop with providing `embedded-io` implemented structure
and `current_time` im milliseconds. It makes `Node` to interact with outer world.
With out call this method in a loop - the node will stop working.

`main.rs`:
```
loop {
    let current_time = Instant::now()
        .duration_since(program_start_time)
        .as_millis() as ms;

    let _ = mesh_node.update(&mut serial, current_time);
}
```

## Reduce packet collisions
It is recommended to set `listen_period` value on multiple devices different from each other,
like:
- device 1 - 230 ms,
- device 2 - 240 ms,
- device 3 - 250 ms,
this will reduce chance of the network to sychronize,
and shall make less packet collisions.
You can play with this values in order to reduce the chance of packet collisions.

### Note: The higher count of nodes in the network leads to the more network stability, but listen period must be higher in order to let devices to share same ether with less collisions. In the stable networks - there is less need to use `transaction` or `ping_pong` sending, unless, you send something very important.

## No encryption
This protocol does not provide data encryption. To secure your data from
being stolen, you should implement (de/en)cryption mechanisms independently.

All nodes must have the same version of the protocol installed to
communicate. Different implementations of the `Packet` structure, or
serialization or deserealization methods
will lead to communication issues.

## Note
Under the hood, data is packed into a `Packet` instance. 
If you need customize packets for your needs - you need configure the `Packet`
`./src/mesh_lib/node/packet/mod.rs` and `./src/mesh_lib/node/packet/types.rs`
Also serialization and deserealization part shall be changed too.

It is also Pure version of protocol released. It is made to cut memory usage
even more. It is partially compatible with embedded-nano-mesh protocol.
Pure version is located in "pure" branch.

## Support project:
You can support project by
donate to bitcoin address: bc1qc50tm0ppj3hh7fecd6d0rv8tdygy8uhe2cemzt
Or you can buy me a coffee:
[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/boshtannik)

## License
This project is licensed under:

- GNU General Public License, Version 3.0 ([LICENSE-GPL](LICENSE-GPL) or [GPL License](https://www.gnu.org/licenses/gpl-3.0.html))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT License ([LICENSE-MIT](LICENSE-MIT) or [MIT License](http://opensource.org/licenses/MIT))

You can choose the license that best suits your preferences.

## Contribution
You can contribute to this project by make fork of 'main' branch and then creating
pyull request to this repository.
Pull request shall be created with next data mentioned.

- Name of the issue the the pull request solves.
- Link to the issue in the pull request description.
- Abstract description of the cause of problem and the way it was solved.
- Optionally notes or wishes for further maintainance or improvement.
- Before pushing the pull request - merge it with main branch again to void all possible conflicts.
- Push the pull request.

# Current protocol configuration:
* Amount of data that protocol can transfer is 32 bytes.
* Amount of addresses in the network is 255. (More than 1 device can have same address)
* Amount of packets that node can store in received queue is 5.
* Amount of packets that node can store in transit queue is 5.
* Amount of data about echoed packets to ignore is: 8.
* Device keeps data about packet to ignore for 1000 ms.

