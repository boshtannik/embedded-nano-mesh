# Mesh Network Protocol for embedded devices
This is the radio mesh network protocol. It is designed to be lightweight,
portable to many plaftorms, and is easy to use. The protocol uses serial
port of your MCU in order to interact with radio module connected to it.
The protocol allows to use any kind of radio module with USART interface,
so you can build mesh node using any kind of radio module with USART
interface and MCU.

MCU - stands for Microcontroller Computer Unit. (Arduino, Raspberry Pi, PC, etc.)

## Node architecture:

 (Library code runs here)
        |
        |
        V
+----------------+              +-----------------+                                  
|                |     USART    |                 |                                  
|      MCU       |<------------>|   Radio module  |                               
|                |              |                 |                                  
+----------------+              +-----------------+                                  
                                                                                        

The protocol has been tested with radio modules JDY-40, and
potentially can use radio modules with similar UART interface,
which might be:
- JDY-41
- SV-610
- HC-11
- HC-12
- LC12S
- GT-38
- LoRa modules

## Goal:
The goal of this project is to provide ability to build easy to use,
mesh - architecture, data transferring protocol for cheap, low memory, components.
This protocol can be used for:
- Home automation
- Remote control
- Remote monitoring (telemetry)
- Decentralized messaging
- etc.

## Working principle:
### The way, how the protocol spreads the data:
The protocol routes the packets in the most dumb way.
It spereads the packet in the way, similar to the spread of the wave
in the pool. It means, that the packet is sent to the nearest devices,
and during the routing by the device - router determines if the packet is reached it's
destination or has to be transitted further with decrease of `lifetime` value of the packet.
Once `lifetime` value is reached zero during routing - the packet is destroyed
in the exact device which routes it. 

The packets, that were just sent by user by `send`, `send_ping_pong` or `send_with_transaction` method
in the same device - bypasses routing and are going directly into the sending queue.
So the message can be sent with `lifetime` set to `0`, anyway it will be transmitted
in the ether for the first time.
Sending of packets from the queue happends during call of `update` method.

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

### How the protocol avoid packet duplication:
During sending of the packet - it is offered to set `ignore_duplicates` parameter
to `true` to prevent network from being jammed by duplicated packets.
It works in the next way: 
Once intermediate node receives the packet with `ignore_duplicates` flag set to `true`,
- it remembers the address of sender of the packet and id of the packet for the specified period of time.
- if the same packet is sent again - it will be ignored by the node.
It leads protocol to spread one exact packet trough the network only once.

Special purpose packets as Ping-Pong or Transaction packets are always
with `ignore_duplicates` flag set to `true` by default.

## Status:
The following functionalities of protocol have been tested and verified:
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
So the library is not platform dependent.

So to port the protocol to the new platform, you need to implement these two generic interfaces.
- `PlatformSerial` - interface to communicate with radio module over USART.
- `PlatformMillis` - interface to track of time.

In case - if you have implemented `PlatformSerial` and `PlatformMillis` trait for your platform,
you can contact developers of this library to include links for your implementations in this README.
Contacs are:
- Telegram channel: [https://t.me/embedded_nano_mesh](https://t.me/embedded_nano_mesh)
- Github: [https://github.com/boshtannik](https://github.com/boshtannik)
This will help this project grow.

In case, if implementations are already present for platform, you need -
you just simply include those and use them into your project.
1 - Include `platform-serial` and `platform-millis` in your project.
`Cargo.toml`:
```
embedded-nano-mesh = "1.0.5"
platform-millis-arduino-nano = { git = "https://github.com/boshtannik/platform-millis-arduino-nano.git", rev = "..." }
platform-serial-arduino-nano = { git = "https://github.com/boshtannik/platform-serial-arduino-nano.git", rev = "..." }
```
2 - Include `platform-serial` and `platform-millis` in your project.
`src/main.rs`:
```
use embedded_nano_mesh::*;

use platform_millis_arduino_nano::{init_timer, ms, Atmega328pMillis};
use platform_serial_arduino_nano::{init_serial, ArduinoNanoSerial};
```

3 - Use `PlatformSerial` and `PlatformMillis` implementations:
```
    /// Send ping-pong example:
    match mesh_node.send_ping_pong::<Atmega328pMillis, ArduinoNanoSerial>( ... ) { ... }

    /// Send with transaction example:
    match mesh_node.send_with_transaction::<Atmega328pMillis, ArduinoNanoSerial>( ... ) { ... }

    /// Update example.
    let _ = mesh_node.update::<Atmega328pMillis, ArduinoNanoSerial>();
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

## Usage
The central component of this protocol is the `Node` structure, which offers interface for
actions like send, receive, broadcast, ping-pong, and send message with transaction.
The `Node` should be constantly updated by
call its `update` method, it - does all internal work:
- routes packets trough the network, transits packets that were sent to other devices, handles `lifetime` of packets.
- handles special packets like `ping` and `pong`, or any kind of transaction one.
- saves received packets that wil lbe available trough `receive` method.
- sends packets, that are in the `send` queue.

To initialize a `Node`, you need to provide `NodeConfig` with values:
- `ExactAddressType`: Sets the device's identification address in the node pool.
- `listen_period`: Sets period in milliseconds that determines how long the device will wait before transmitting on the network. It prevents network congestion.

You can regulate the number of hops that the packet will be able to
make - by configuring the `lifetime` during making the send
of the message. For example:
- setting `lifetime` to 1 will limit the message's reach to the nearest devices in the network.
- setting `lifetime` to 10 will make the packet able to pass 10 nodes before being destroyed.

To send the message to all nodes in the network, you can
send it with standard `send` method, and put `GeneralAddressType::BROADCAST` as the
`destination_device_identifier`. Every device will treat `GeneralAddressType::BROADCAST`
as it's own address, will keep the message as received and will transit copy of that message further.

To send the message to a specific device in the network, you can
send it with standard `send` method, and put `GeneralAddressTyp::ExactAddressType(...)` as the
`destination_device_identifier`.

The term "echoed message" refers to a duplicated message that has
been re-transmitted into the ether by an intermediate device.

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
fields in `src/Node/packet/config.rs` and `src/Node/packet/types.rs`.
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
