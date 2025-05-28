# Mesh Network Protocol for embedded devices
Mesh network protocol. Lets build mesh network out any kind
of MCU device + Radio module device into a mesh node. It is designed
to be lightweight, easy to use and portable to many plaftorms. The protocol
may use variety of radio modules.

The network extends and heals itself automatically by communicating
with other nodes, which have same protocol version installed. Most
versions of this protocol are compatible, but for the best performance
it is recommended to use same and the latest version.

MCU - stands for Microcontroller Computer Unit. (Arduino, Raspberry Pi, PC, etc.)

## Cook recipe:
```
                           One of radio modules:
       One of MCU:          +----------------+
  +----------------+        | JDY-41         |
  |                |        | SV-610         |
  | Arduino        |        | HC-11          |
  | Raspberry      |   +    | HC-12          |    =    Mesh node
  | Linux PC       |        | LC12S          |
  | etc.           |        | GT-38          |
  +----------------+        | LoRa modules   |
                            +----------------+
```

## Goal:
The goal of this project is to provide ability to build mesh
network any even with low memory, cheap and easily accessible microcontollers + radio modules
This protocol can be used for:
- Home automation
- Remote control
- Remote monitoring (telemetry)
- Decentralized messaging
- etc.

## Quick links to usage examples:
- [arduino-nano](https://github.com/boshtannik/embedded-nano-mesh-arduino-nano-example)
Sometimes code binary might not fit onto your arduino board memory, in order to
reduce the size of final binary - it is recommended to compile it with
--release flag - it increases optimisation level tlat leads to smaller binary.
- [linux](https://github.com/boshtannik/embedded-nano-mesh-linux-example)
- [linux-cli-tool](https://github.com/boshtannik/embedded-nano-mesh-cli-tool)

## Working principle:
### The way, how the protocol spreads the data:
The protocol routes the packets in the most dumb way.
It spereads the packet in the way, similar to the spread of the atenuating
wave in the pool. It means, that all near devices, that can catch the packet - cathes it.
Then the router of the node - determines if the packet is reached it's
destination or the packet has to be transitted further into the network.

You can regulate how far (how many devices) the message will pass
by set the `lifetime` during the sending.

While packet is travels trough the network - Lifetime of packet is decreased by intermediate device.
Once `lifetime` value is reached zero during travel - the packet gets destroyed
by the exact device which currently transits it.

Packet with lifetime set to 0 or 1 - will be broadcasted into the network only once.

Node have 2 internal queues, queues are not exposed to user:
- `send` - for packets that node holds to be sent.
- `transit` - for packets, that node holds to be transitted from other devices.
Sizes of those queues are configurable. And their both configuration of size
is made by `PACKET_QUEUE_SIZE` constant in `./src/mesh_lib/node/constants.rs`.

### How the protocol avoid packet duplication:
During send of the packet using `send_to_exact` method - you can set `filter_out_duplication` parameter
to prevent network from being jammed by duplicated packets.
Methods `send_ping_pong`, `broadcast`, `send_with_transaction` has this parameter set by default.
As long as `send_ping_pong` and `send_with_transaction` needs more than one packet to be sent trough the network,
and `broadcast` without `filter_out_duplication` just jams the whole network by echoed packets.

The `filter_out_duplication` works in the next way:
1 - Node sets `ignore_duplication` flag to the packet flags.
2 - Once intermediate node receives the packet with `ignore_duplication` flag set to `true`,
2.1 it remembers the `sender_device_identifier` of the packet and `id` of the packet for the
specified `RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD` period of time. if the packet with same
`sender_device_identifier` and with same `id` is received again by that same node - node
ignores packet for that specified period of time.
 
`RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD` period of time. This period is configurable in `./src/mesh_lib/node/constants.rs`.

## Status:
* The version is: 2.1.5:
  Every planned functionality is working. It is:
  - Send data.
  - Receive data.
  - Send data with ignorance of duplicated packets.
  - Send data with limited of number of hops.
  - Broadcast data to all nodes.
  - Message transition by the intermediate nodes.
  - Send data via Ping-pong method, and receive result saying that ping-pong send finished.
  - Send data via Transaction and receive result saying that transaction being finished.
* Fully backward compatible with version 2.0.0
* Transaction of Backward compatibility with version 1.0.0 is restored. Only backward compatibility with version 2.1.0 is broken.
* If you want to test this version of protocol with other version - you can import different versions of that protocol and do tests in the simulation as it is done in usage tests.

## Cross-platform compatibility
Protocol currently natively runs on:
- Arduino nano
- Linux (Raspberry PI, Desktop)

May work natively on:
- Windows
- Mac
- STM32
- ESP8266
- Raspberry PI pico

## Porting to other platforms
Initially it was designed to be able to run on Arduino nano - it can run
on huge variety of other microcontollers or personal computers natively.
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

## Manage packet collisions
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


# Current protocol configuration:
* Amount of data that protocol transfers is 32 bytes.
* Amount of addresses in the network is 255. (More than 1 device can have same address)
* Amount of packets that node can store in received queue is 5.
* Amount of packets that node can store in transit queue is 5.
* Amount of data about echoed packets to ignore is: 8.
* Device keeps data about packet to ignore for 1000 ms.

Under the hood, data is packed into a `Packet` instance. 
If you need customize protocol for your needs - you need configure the `Packet` or it's types.
Serialization might be touched also.
```
`./src/mesh_lib/node/packet/mod.rs`                              -  File with definition of structure of `Packet`.
`./src/mesh_lib/node/packet/types.rs`                            -  File with definition of types of fields of `Packet`.
`./src/mesh_lib/node/packet/traits/serializer.rs`                -  File with definition of `Packet` serialization interface.
`./src/mesh_lib/node/packet/trait_implementations/serializer.rs` -  File with implementation of `Packet` serialization interface.
`./src/mesh_lib/node/packet/constants.rs`                        -  File with definition of size of transferred data.
`./src/mesh_lib/node/constants.rs` -                             -  File contains presets for queue sizes,
                                                                    packet filter presets.
```

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

