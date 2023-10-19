# Arduino Nano Mesh

## Goal

The goal of this project is to create a mesh-like data transferring protocol using cheap and simple components. This protocol allows you to build a reliable and easy-to-use mesh-like network for various applications, such as:

- Home automation
- Remote control
- Remote monitoring (telemetry)

While initially designed for Atmega328p chips, the code is tied to this platform but can be forked and ported to other platforms.

## Status

The code is designed to utilize UART rx/tx pins of your MCU and has been tested with popular radio modules like JDY-40, JDY-41, SV-610, HC-11, HC-12, LC12S, and GT-38. The following functionalities have been tested and verified:

- Sending data
- Receiving data
- Broadcasting
- Message transit
- Ping-Pong sending
- Transaction sending

## Warning

This protocol does not provide data encryption. To secure your data from being stolen, you should implement encryption and decryption mechanisms independently.

## Usage
### Receiver
```
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use mesh_lib::{init_node, DeviceIdentifier, NodeConfig};
use panic_halt as _;

mod mesh_lib;

use mesh_lib::millis::ms;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut mesh_node = init_node(NodeConfig {
        device_identifier: DeviceIdentifier(2),
        listen_period: 150 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_timer: dp.TC0,
    });

    loop {
        let _ = mesh_node.update();

        if let Some(got_message) = mesh_node.receive() {
          for byte in got_message.data {
            /* Do your job here */
          }
        }
    }
}
```

### Send from device 1 to device 2 (allow packet do 10 hops)
```
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use mesh_lib::{init_node, DeviceIdentifier, LifeTimeType, NodeConfig};
use panic_halt as _;

mod mesh_lib;

use mesh_lib::millis::{millis, ms};

use mesh_lib::NodeString;
use ufmt::uwrite;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut mesh_node = init_node(NodeConfig {
        device_identifier: DeviceIdentifier(1),
        listen_period: 150 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_timer: dp.TC0,
    });

    let mut last_send_time: ms = millis();
    let mut now_time: ms;
    let mut packet_counter: u32 = 0;

    loop {
        let _ = mesh_node.update();

        now_time = millis();

        if now_time > (last_send_time + 210 as ms) {
            let mut message = NodeString::new();
            uwrite!(&mut message, "Packet #: {}", packet_counter).unwrap();

            mesh_node
                .send(
                    message.into_bytes(),
                    DeviceIdentifier(2),
                    10 as LifeTimeType,
                    true,
                )
                .unwrap_or_else(|_| serial_debug!("Not sent!"));

            last_send_time = now_time;
            packet_counter = packet_counter.overflowing_add(1).0;
        }
    }
}
```
### Broadcast message to all near devices (1 hop)
Number of hops, sets ammount - for how many devices the packet will be able to jump trough.
In that case, the packet will travel only to the nearest devices.
`BROADCAST_RESERVED_IDENTIFIER` - is the identifier, that is reserved in the protocol
                                  by every device in the network to be treated as it's own.
```
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use mesh_lib::node::BROADCAST_RESERVED_IDENTIFIER;
use mesh_lib::{init_node, DeviceIdentifier, LifeTimeType, NodeConfig};
use panic_halt as _;

mod mesh_lib;

use mesh_lib::millis::{millis, ms};

use mesh_lib::NodeString;
use ufmt::uwrite;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut mesh_node = init_node(NodeConfig {
        device_identifier: DeviceIdentifier(1),
        listen_period: 150 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_timer: dp.TC0,
    });

    let mut last_send_time: ms = millis();
    let mut now_time: ms;
    let mut packet_counter: u32 = 0;

    loop {
        let _ = mesh_node.update();

        now_time = millis();

        if now_time > (last_send_time + 210 as ms) {
            let mut message = NodeString::new();
            uwrite!(&mut message, "Packet #: {}", packet_counter).unwrap();

            mesh_node
                .send(
                    message.into_bytes(),
                    DeviceIdentifier(BROADCAST_RESERVED_IDENTIFIER),
                    1 as LifeTimeType,
                    true,
                )
                .unwrap_or_else(|_| serial_debug!("Not sent!"));

            last_send_time = now_time;
            packet_counter = packet_counter.overflowing_add(1).0;
        }
    }
}
```
### Sending transaction from device 1 to device 2
The transaction initialtor will wait for 3 seconds to get transaction done.
In case of transcation is not done in that period of time - error result will be returned.
It is better to use `ignore_duplication` flag in order to void ether being jammed by
transaction packets.
```
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use mesh_lib::{init_node, DeviceIdentifier, LifeTimeType, NodeConfig};
use panic_halt as _;

mod mesh_lib;

use mesh_lib::millis::{millis, ms};

use mesh_lib::NodeString;
use ufmt::uwrite;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut mesh_node = init_node(NodeConfig {
        device_identifier: DeviceIdentifier(1),
        listen_period: 150 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_timer: dp.TC0,
    });

    let mut last_send_time: ms = millis();
    let mut now_time: ms;
    let mut packet_counter: u32 = 0;

    loop {
        let _ = mesh_node.update();

        now_time = millis();

        if now_time > (last_send_time + 210 as ms) {
            let mut message = NodeString::new();
            uwrite!(&mut message, "Packet #: {}", packet_counter).unwrap();

            mesh_node
                .send_with_transaction(
                    message.into_bytes(),
                    DeviceIdentifier(2),
                    10 as LifeTimeType,
                    true,
                    3000 as ms,
                )
                .unwrap_or_else(|_| serial_debug!("Transaction failed"));

            last_send_time = now_time;
            packet_counter = packet_counter.overflowing_add(1).0;
        }
    }
}
```

## Possible Use Case

You can broadcast encrypted messages to the entire network, allowing devices capable of decryption to react to these messages. In other words, it resembles a "Publisher/Subscriber" pattern.

## Main Components

The central component of this protocol is the `Node` structure, which offers a user-friendly interface for actions like sending, receiving, broadcasting, ping-ponging, and handling message transactions. The `Node` should be constantly updated by calling its `update` method.

To initialize a `Node`, you need to provide two values:

1. `DeviceIdentifier`: Represents the device's identification address in the node pool.
2. `Listen period`: A value in milliseconds that determines how long the device will wait before transmitting on the network to prevent network congestion.

You can regulate the range of packets by configuring the `lifetime` parameter. For example, setting `lifetime` to 1 will limit the message's reach to the nearest devices in the network.

The term "echoed message" refers to a duplicated message that has been re-transmitted into the ether by an intermediate device.

### Send Method

The `send` method requires the following arguments:

- `data`: A `PacketDataBytes` instance to hold the message bytes.
- `destination_device_identifier`: A `DeviceIdentifier` instance indicating the target device.
- `lifetime`: A `LifeTimeType` instance to control the message's travel distance.
- `filter_out_duplication`: A boolean flag to filter out echoed messages from the network.

### Receive Method

The `receive` method optionally returns received data in a `PacketDataBytes` instance.

### Send Ping-Pong Method

The `send_ping_pong` method sends a message with a "ping" flag to the destination node and waits for the same message with a "pong" flag. It returns an error if the ping-pong exchange fails. The following arguments are required:

- `data`: A `PacketDataBytes` instance.
- `destination_device_identifier`: A `DeviceIdentifier` instance.
- `lifetime`: A `LifeTimeType` instance.
- `filter_out_duplication`: A boolean flag.
- `timeout`: An `ms` instance specifying how long to wait for a response.

### Send with Transaction Method

The `send_with_transaction` method sends a message and handles all further work to ensure the target device responds. It returns an error if the transaction fails. The required arguments are:

- `data`: A `PacketDataBytes` instance.
- `destination_device_identifier`: A `DeviceIdentifier` instance.
- `lifetime`: A `LifeTimeType` instance.
- `filter_out_duplication`: A boolean flag.
- `timeout`: An `ms` instance to specify the response wait time.

## Note

Under the hood, data is packed into a `Packet` instance. You can configure the `Packet` data fields in `src/Node/packet/config.rs` and `src/Node/packet/types.rs`.

## Compatibility

All nodes must have the same version of the protocol installed to communicate effectively. Different device implementations of the `Packet` structure may lead to communication issues.

## Getting Started

1. Install the required prerequisites, as described in the [`avr-hal` README](https://github.com/Rahix/avr-hal#readme) (avr-gcc, avr-libc, avrdude, [`ravedude`](https://crates.io/crates/ravedude)).

2. Build the firmware with `cargo build`.

3. Flash the firmware to a connected board with `cargo run`. If `ravedude` fails to detect your board, consult its documentation at [crates.io](https://crates.io/crates/ravedude).

4. `ravedude` will open a console session after flashing for UART console interaction.

## License

This project is licensed under:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT License ([LICENSE-MIT](LICENSE-MIT) or [MIT License](http://opensource.org/licenses/MIT))

You can choose the license that best suits your preferences.

## Contribution

Unless you specify otherwise, any contributions submitted for inclusion in this project, as defined in the Apache-2.0 license, will be dual-licensed under both licenses without additional terms or conditions.
