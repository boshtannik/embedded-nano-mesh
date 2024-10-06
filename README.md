# Mesh Network Protocol for embedded devices
This is the low speed mesh network protocol. It allows to turn almost any
kind of MCU device + Radio module device into a mesh node. It is designed
to be lightweight, easy to use and portable to many plaftorms. The protocol
uses serial port of your MCU in order to communicate with other nodes
via radio modules connected to it.

The network extends and heals itself automatically by communicating
with other nodes, which have same protocol version installed. Most
versions of this protocol are compatible, but for the best performance
it is recommended to use the latest version.

The protocol has been tested with radio
modules JDY-40 during the development, and potentially can
use radio modules with similar UART interface,
which might be:
- JDY-41
- SV-610
- HC-11
- HC-12
- LC12S
- GT-38
- LoRa modules

MCU - stands for Microcontroller Computer Unit. (Arduino, Raspberry Pi, PC, etc.)

## Node architecture:
```
 (Library code runs here which turns MCU into a mesh node)
        |                                          
        |                                          
        V                                          
+----------------+              +-----------------+
|                |     USART    |                 |
|      MCU       |<------------>|   Radio module  |
|                |              |                 |
+----------------+              +-----------------+
```

## Network architecture:
```
+----------------+               +-----------------+                 +----------------+ 
|                |               |                 |                 |                | 
|      Node      |  (( Ether ))  |      Node       |   (( Ether ))   |      Node      |  
|   Address: 1   |               |   Address: 2    |                 |    Address: 3  | 
|                |               |                 |                 |                | 
+----------------+  <--------->  +-----------------+  <----------->  +----------------+ 
                  ^               ^                                                    
                   \             /                                                     
                    \           /                                                      
       (( Ether ))   \         /   (( Ether ))                                                    
                      \       /                                                        
                       \     /                                                         
                        v   v                                                           
              +----------------+                                                       
              |                |                                                       
              |      Node      |                                                       
              |   Address: 4   |                                                       
              |                |                                                       
              +----------------+                                                       
```

## Quick links to usage examples:
- [arduino-nano](https://github.com/boshtannik/nano-mesh-arduino-nano-example)
- [linux](https://github.com/boshtannik/nano-mesh-linux-example)

## Goal:
The goal of this project is to provide ability to build easy to use,
mesh - architecture, data transferring network out of cheap, low memory, components.
This protocol can be used for:
- Home automation
- Remote control
- Remote monitoring (telemetry)
- Decentralized messaging
- etc.

## Working principle:
### The way, how the protocol spreads the data:
The protocol routes the packets in the most dumb way.
It spereads the packet in the way, similar to the spread of the atenuating
wave in the pool. It means, that all near devices, that can catch the packet - cathes it.
Then the device's router - determines if the packet is reached it's
destination or has to be transitted further with decrease of `lifetime` value of the packet.
Once `lifetime` value is reached zero during routing - the packet is destroyed
in the exact device which currently routes it. 

The packets, that were just sent by user by `send`, `send_ping_pong` or `send_with_transaction` method
in the device, which performs the operation - that packets bypasses routing and are sent directly into
sending queue, and then into the ether. Meaning that lifetime of the packet is not decreased by the router.
So the message can be sent even with `lifetime` set to `0`, anyway it will be transmitted
in the ether for the first time.
Sending of packets from the queues happends during call of `update` method.

It means, that the user can send the message with:
* Set the `lifetime` to `0`, and the packet will be transmitted into the ether,
  nearest device will receive it, check if the destination is reached.
  If the destination is reached - catch the data.
  Otherwise - try to transmit further with decrease of `lifetime` value which
  will lead to packet destruction due to the end of packet's `lifetime`.

* Also set the `lifetime` to `1`, and the packet will be transmitted into the ether,
  nearest device will receive it, check if the destination is reached,
  If the destination is reached - catch the data.
  Otherwise - try to transmit further with decrease of `lifetime` value which
  will lead to packet destruction due to the same reason.

* Set the `lifetime` to `2` and the packet will be transmitted into the ether,
  nearest device will receive it, check if the destination is reached,
  If the destination is reached - catch the data.
  Otherwise - try to transmit further with decrease of `lifetime` value which
  will lead packet transition back into the ether, but with less `lifetime` value.

Every node have 2 queues:
- `send` - for sending packets
- `transit` - for packets, that are sent to the other devices.
Sizes of those queues are configurable. And their both configuration of size
is made by `PACKET_QUEUE_SIZE` constant in `./src/mesh_lib/node/constants.rs`.

* And so on..

### How the protocol avoid packet duplication:
During sending of the packet - it is offered to set `filter_out_duplication` parameter
to `true` to prevent network from being jammed by duplicated packets.
It works in the next way: 
Once intermediate node receives the packet with `ignore_duplicates` flag set to `true`,
- it remembers the `sender_device_identifier` of the packet and `id` of the packet for the `RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD`. - This period is configurable.
- if the same packet with same `sender_device_identifier` and with same `id` is sent again - it will be ignored by the node.
It leads protocol to spread one exact packet trough the network only once.

Special purpose packets as Ping-Pong or Transaction packets are always
with `ignore_duplicates` flag set to `true` by default.

## Status:
Every planned functionality is working. It is:
- Send data.
- Receive data.
- Send data with ignorance of duplicated packets.
- Send data with limited of number of hops.
- Broadcast data to all nodes.
- Message transition by the intermediate nodes.
- Send data with ping flag, and receive answer with pong flag set.
- Send data via Transaction and receive packet about transaction being finished.

## Cross-platform compatibility
The mesh metwork was tested using few Arduino nano boards and one Linux machine within the same network.

For now the protocol was tested on:
- Arduino nano
- Linux (Raspberry PI, Desktop)

Potentially can be ported to:
- Windows
- Mac
- STM32
- ESP8266
- Raspberry PI pico

## Porting to other platforms
While initially designed to be able to run at least on
Atmega328p chips - it can be ported to huge variety of other platforms.
Platforms shall be supported by `embedded-hal`.

Also this protocol is welcomed to be ported on other platforms.
In order to simplify process of porting of this protocol to the new
platforms - the common behaviour is moved out of implementation
to make it interchangable with implementations of PlatformMillis and PlatformSerial traits for
each other platform.

To be able to run protocol on new platforms, it is needed to have implemented
`PlatformMillis` and `PlatformSerial` traits for this specific platform.
- `PlatformSerial` - trait to communicate with radio module over USART.
- `PlatformMillis` - trait to track of time since program start.

In case - if you have implemented `PlatformSerial` and `PlatformMillis` trait for your platform,
you can contact developers of this library to include links for your implementations in this README.
Contacs are:
- Telegram channel: [https://t.me/embedded_nano_mesh](https://t.me/embedded_nano_mesh)
- Github: [https://github.com/boshtannik](https://github.com/boshtannik)
This will help this project grow.


## Usage steps:
In case, if implementations are already present for platform, you need -
just follow three steps to use it: (Examples are for arduino nano)
1 - Include `platform-serial` and `platform-millis` in your project.
`Cargo.toml`:
```
embedded-nano-mesh = "1.0.5"
platform-millis-arduino-nano = { git = "https://github.com/boshtannik/platform-millis-arduino-nano.git", rev = "..." }
platform-serial-arduino-nano = { git = "https://github.com/boshtannik/platform-serial-arduino-nano.git", rev = "..." }
```
2 - Include `platform-serial` and `platform-millis` in your project, and of course the library `embedded-nano-mesh` itself.
`src/main.rs`:
```
use embedded_nano_mesh::*;

use platform_millis_arduino_nano::{init_timer, ms, Atmega328pMillis};
use platform_serial_arduino_nano::{init_serial, ArduinoNanoSerial};
```

3 - Use `PlatformSerial` and `PlatformMillis` implementations during use of dependent methods on this traits:
`src/main.rs`:
```
/// Send ping-pong example:
match mesh_node.send_ping_pong::<Atmega328pMillis, ArduinoNanoSerial>( ... ) { ... }

/// Send with transaction example:
match mesh_node.send_with_transaction::<Atmega328pMillis, ArduinoNanoSerial>( ... ) { ... }

/// Update example.
loop {
  let _ = mesh_node.update::<Atmega328pMillis, ArduinoNanoSerial>();
}
```
Full examples are available below.

## Arduino nano port.
The implementation of PlatformSerial for Arduino nano board is done by:
- [platform-serial-arduino-nano](https://github.com/boshtannik/platform-serial-arduino-nano)

The implementation of PlatformMillis for Arduino nano board is done by:
- [platform-millis-arduino-nano](https://github.com/boshtannik/platform-millis-arduino-nano)

Usage examples can be found here:
- [arduino-nano](https://github.com/boshtannik/nano-mesh-arduino-nano-example)

Sometimes code binary might not fit onto your arduino board memory, in order to
reduce the size of final binary - it is recommended to compile it with
--release flag - it increases optimisation level tlat leads to smaller binary.

## Linux port.
The implementation of PlatformSerial for Linux is done by:
- [platform-serial-linux](https://crates.io/crates/platform-serial-linux)

The implementation of PlatformMillis for Linux is done by:
- [platform-millis-linux](https://crates.io/crates/platform-millis-linux)

Usage examples can be found here:
- [linux](https://github.com/boshtannik/nano-mesh-linux-example)

## Extended usage description
1 - Instantiate `Node` structure.
2 - Constantly call `update` method of `Node`.
3 - Call any method of `Node` structure you need, such as:
  - `send`
  - `receive`
  - `send_ping_pong`
  - `send_with_transaction`

To initialize a `Node`, you need to provide `NodeConfig` with values:
- `ExactAddressType`: Sets the device's identification address in the node pool.
- `listen_period`: Sets period in milliseconds that determines how long the device will wait before transmitting on the network. It prevents network congestion.

`main.rs`:
```
let mut mesh_node = Node::new(NodeConfig {
    device_address: ExactAddressType::new(1).unwrap(),
    listen_period: 150 as ms,
});
```

The central component of this protocol is the `Node` structure, which offers interface for
actions like `send`, `receive`, `send_ping_pong`, and `send_with_transaction`.
The `Node` should be constantly updated by
call its `update` method, it - does all internal work:
- routes packets trough the network, transits packets that were sent to other devices, handles `lifetime` of packets.
- handles special packets like `ping` and `pong`, or any kind of transaction one.
- saves received packets that wil lbe available trough `receive` method.
- sends packets, that are in the `send` queue.

As the protocol relies on physical environment - it is crucial to provide
ability to the library to rely on time counting and on USART interface, as
it is described above by `PlatformSerial` and `PlatformMillis` implementations.

During the use of methods, that relies on `PlatformMillis` trait and `PlatformSerial` trait -
it is needed to provide those implementations during the method call.
Those methods are:
- `update`
- `send_ping_pong`
- `send_with_transaction`
More examples can be found in the examples repositories above.

To send the message to all nodes in the network, you can
send it with standard `send` method, and put `GeneralAddressType::Broadcast` as the
`destination_device_identifier`. Every device will treat `GeneralAddressType::Broadcast`
as it's own address, will keep the message as received and will transit copy of that message further.
`main.rs`:
```
mesh_node.send(
    NodeString::from("Hello, world!").into_bytes(),
    GeneralAddressType::Broadcast,
    3.into(),
    true,
);
```

To send the message to a specific device in the network, you can
send it with standard `send` method, and put `GeneralAddressType::Exact(...)` as the
`destination_device_identifier`.
`main.rs`:
```
mesh_node.send(
    NodeString::from("Hello, world!").into_bytes(),
    GeneralAddressType::Exact(ExactAddressType::new(3).unwrap()),
    3.into(),
    true,
);
```

The term "echoed message" refers to a duplicated message that has
been re-transmitted into the ether by an intermediate device.

You can regulate the number of hops that the packet will be able to
make - by configuring the `lifetime` during making the send
of the message. For example:
- setting `lifetime` to 1 will limit the message's reach to the nearest devices in the network.
- setting `lifetime` to 10 will make the packet able to pass 10 nodes before being destroyed.

### Receive Method
The `receive` method optionally returns received data in a `PacketDataBytes` instance in case
if that packet was previously received by this exact device.

### Send Method
The `send` method requires the following arguments:

- `data`: A `PacketDataBytes` instance to hold the message bytes.
- `destination_device_identifier`: A `GeneralAddressType` instance indicating exact target device while using GeneralAddressType::Exact(...) or indicating all devices to receive the message by using GeneralAddressType::BROADCAST.
- `lifetime`: A `LifeTimeType` instance to control for how far the message can travel.
- `filter_out_duplication`: A boolean flag to filter out echoed messages from the network.

### Send Ping-Pong Method
The `send_ping_pong` method sends a message with a "ping" flag to the destination node and
waits for the same message with a "pong" flag. It returns an error if the ping-pong exchange fails.
The following arguments are required:

- `data`: A `PacketDataBytes` instance.
- `destination_device_identifier`: A `ExactAddressType` instance, that indicates exact target device address.
- `lifetime`: A `LifeTimeType` instance.
- `timeout`: An `ms` instance specifying how long to wait for a response.

### Send with Transaction Method
The `send_with_transaction` method sends a message and handles all further work to
ensure the target device have received it correctly. It returns an error if the transaction failed.
The required arguments are:

- `data`: A `PacketDataBytes` instance.
- `destination_device_identifier`: A `ExactAddressType` instance, that indicates exact target device address.
- `lifetime`: A `LifeTimeType` instance.
- `timeout`: An `ms` instance to specify the response wait time.

### Update Method
The `update` method is used to perform all internal operation of the `Node`.
It shall be called in a loop with providing `PlatformMillis` and `PlatformSerial` instances
- it allows `Node` to interact with MCU peripherals, such as time counting and USART.

## Reduce packet collisions
It is recommended to set `listen_period` value on multiple devices different from each other,
like:
- device 1 - 230 ms,
- device 2 - 240 ms,
- device 3 - 250 ms,
this will reduce chance of the network to sychronize,
which will lead to packet collisions.
You can play with this values in order to reduce the chance of packet collisions.

### Note: The higher count of nodes in the network leads to the more network stability. In the stable networks - there is less need to use `transaction` or `ping_pong` sending, unless, you send something very important.

## Warning
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

## License
This project is licensed under:

- GNU General Public License, Version 3.0 ([LICENSE-GPL](LICENSE-GPL) or [GPL License](https://www.gnu.org/licenses/gpl-3.0.html))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT License ([LICENSE-MIT](LICENSE-MIT) or [MIT License](http://opensource.org/licenses/MIT))

You can choose the license that best suits your preferences.

## Contribution
Unless you specify otherwise, any contributions submitted for inclusion in this project, as defined in the Apache-2.0 license, will be dual-licensed under both licenses without additional terms or conditions.

## Donation
Also you can support the project by donating few
bucks on the bitcoin address: bc1qc50tm0ppj3hh7fecd6d0rv8tdygy8uhe2cemzt
