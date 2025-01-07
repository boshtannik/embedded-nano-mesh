# Mesh Network Protocol for embedded devices
This is the low speed mesh network protocol. It allows to turn almost any
kind of MCU device + Radio module device into a mesh node. It is designed
to be lightweight, easy to use and portable to many plaftorms. The protocol
may use variety of radio modules, implementation of driver is required for
radio modulee connected to MCU.

The network extends and heals itself automatically by communicating
with other nodes, which have same protocol version installed. Most
versions of this protocol are compatible, but for the best performance
it is recommended to use the latest version.

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

## Node architecture:
```
 (Library code runs here which turns MCU into a mesh node)
        |                                          
        |                                          
        V           (Interface                     
+----------------+    of your   +-----------------+
|                |    choice)   |                 |
|      MCU       |<------------>|   Radio module  |
|                |              |                 |
+----------------+              +-----------------+
```

## Network possible architecture:
```
+----------------+               +----------------+                 +---------------+ 
|                |               |                |                 |               | 
|      Node      |  (( Ether ))  |      Node      |   (( Ether ))   |     Node      |  
|   Address: 1   |               |   Address: 2   |                 |   Address: 3  | 
|                |  <--------->  |                |  <----------->  |               | 
+----------------+               +----------------+                 +---------------+ 
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
- [arduino-nano](https://github.com/boshtannik/embedded-nano-mesh-arduino-nano-example)
- [linux](https://github.com/boshtannik/embedded-nano-mesh-linux-example)
- [linux-cli-tool](https://github.com/boshtannik/embedded-nano-mesh-cli-tool)

## Goal:
The goal of this project is to provide ability to build easy to use,
mesh - architecture, data transferring network out of cheap, low memory, components.
This protocol can be used for:
- Home automation
- Remote control
- Remote monitoring (telemetry)
- Decentralized messaging
- etc.
Goal of to be able to use cheap components - is dictated by
need to extend the network by nodes built of cheap components which
may handle simplest logic or no logic at all.

## Support project:
If you earn money with using that code - Please donate
to bitcoin address: bc1qc50tm0ppj3hh7fecd6d0rv8tdygy8uhe2cemzt
to support the project.
Or you can buy me a coffee:
[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/boshtannik)

## Working principle:
### The way, how the protocol spreads the data:
The protocol routes the packets in the most dumb way.
It spereads the packet in the way, similar to the spread of the atenuating
wave in the pool. It means, that all near devices, that can catch the packet - cathes it.
Then the device's router - determines if the packet is reached it's
destination or has to be transitted further with decrease of `lifetime` value of the packet.
Once `lifetime` value is reached zero during routing - the packet gets destroyed
by the exact device which currently routes it.

The packets, that were just sent by user by `send_to_exact`, `broadcast`, `send_ping_pong` or `send_with_transaction`
method in the device, which performs the operation - that packets bypasses routing and are sent directly into
sending queue, and then into the ether. It means that lifetime of the packet is not decreased by the router
of the device. So the message can be sent even with `lifetime` set to `0`, anyway it will be transmitted
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

* And so on..

Every node have 2 internal queues, they are not exposed to user:
- `send` - for sending packets
- `transit` - for packets, that are sent to the other devices.
Sizes of those queues are configurable. And their both configuration of size
is made by `PACKET_QUEUE_SIZE` constant in `./src/mesh_lib/node/constants.rs`.

### How the protocol avoid packet duplication:
During sending of the packet - it is offered to set `filter_out_duplication` parameter
to `true` to prevent network from being jammed by duplicated packets.
It works in the next way: 
Once intermediate node receives the packet with `ignore_duplication` flag set to `true`,
- it remembers the `sender_device_identifier` of the packet and `id` of the packet for the
`RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD`. - This period is configurable. if the packet
with same `sender_device_identifier` and with same `id` is sent again - it will be ignored by
the node for specified period of time which was described above.
`ignore_duplication` leads protocol to spread one exact packet trough the network only once.

Special purpose packets as Ping-Pong or Transaction packets are always
with `filter_out_duplication` flag set to `true` by default, because they
require more than one packet be sent to handle whole process of `ping-pong` or `transaction`.

## Status:
The version is: 2.0.0:
  Every planned functionality is working. It is:
  - Send data.
  - Receive data.
  - Send data with ignorance of duplicated packets.
  - Send data with limited of number of hops.
  - Broadcast data to all nodes.
  - Message transition by the intermediate nodes.
  - Send data via Ping-pong method, and receive result saying that ping-pong send finished.
  - Send data via Transaction and receive result saying that transaction being finished.

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
While initially designed to be able to run at least on
Atmega328p microcontollers - it can run natively on huge variety of other platforms and operating systems.
Protocol is using `embedded-serial` crate to depend on MutBlockingTX and MutNonBlockingRX
traits which shall be implemented and provided during use of the library, for methods
`update`, `send_ping_pong` and `send_with_transaction`.

## Issues and discussions:
Contacs are:
- Telegram channel: [https://t.me/embedded_nano_mesh](https://t.me/embedded_nano_mesh)
- Github: [https://github.com/boshtannik](https://github.com/boshtannik)
This will help this project grow.


## Usage:
1 - Include library.
`Cargo.toml`:
```
embedded-nano-mesh = "2.0.0"
embedded-serial = "0.5.0"
```
2 - Include implementation of embedded-serial or implement it yourself.
`src/main.rs`:
```
struct LinuxInterfaceDriver {
    serial: serialport::TTYPort,
}

impl LinuxInterfaceDriver {
    pub fn new(serial: serialport::TTYPort) -> LinuxInterfaceDriver {
        LinuxInterfaceDriver { serial }
    }
}

impl embedded_serial::MutBlockingTx for LinuxInterfaceDriver {
    type Error = ();

    fn putc(&mut self, ch: u8) -> Result<(), Self::Error> {
        self.serial.write(&[ch]).unwrap();
        Ok(())
    }
}

impl embedded_serial::MutNonBlockingRx for LinuxInterfaceDriver {
    type Error = ();

    fn getc_try(&mut self) -> Result<Option<u8>, Self::Error> {
        let mut buf = [0u8];
        match self.serial.read(&mut buf) {
            Ok(_) => Ok(Some(buf[0])),
            Err(_) => Ok(None),
        }
    }
}

```

3 - initialize and use your implementation of embedded-serial in your code:
`src/main.rs`:
```
    let mut interface = LinuxInterfaceDriver::new( ... );
    let mut mesh_node = ...;

    match mesh_node.send_to_exact(
        message.into_bytes(),              // Content.
        ExactAddressType::new(2).unwrap(), // Send to device with address 2.
        10 as LifeTimeType, // Let message travel 10 devices before being destroyed.
        true,
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
make - by configuring the `lifetime` during making the send
of the message. For example:
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
The central component of this protocol is the `Node` structure, which offers interface for
actions like `send_to_exact`, `broadcast`, `receive`, `send_ping_pong`, and `send_with_transaction`.

The `Node` should be constantly updated by call its `update` method.
During call of `update` method - it does all internal work:
- routes packets trough the network
- transits packets that were sent to other devices
- handles `lifetime` of packets
- handles special packets like `ping` and `pong`, or any kind of transaction one.
- saves received packets that wil lbe available trough `receive` method.
- sends packets, that are in the `send` queue.

As the protocol relies on physical environment - it is crucial to provide
ability to the library to rely on time counting and on communication interface, time
calculation is provided by millis_provider closure, and interface_driver
is described above by `embedded-serial` traits.

During the use of methods, that relies on millis_provider closure and interface_driver which
is the structure that implements embedded-serial trait -
it is needed to provide those implementations during the method call.
Those methods are:
- `update`
- `send_ping_pong`
- `send_with_transaction`

### New Method
To initialize a `Node`, you need to provide `NodeConfig` with values:
- `ExactAddressType`: Sets the device's identification address in the node pool. It is ok to have multiple deivces sharing same address in the same network.
- `listen_period`: Sets period in milliseconds that determines how long the device will wait before transmitting packet to the network. It prevents network congestion.

`main.rs`:
```
let mut mesh_node = Node::new(NodeConfig {
    device_address: ExactAddressType::new(1).unwrap(),
    listen_period: 150 as ms,
});
```

### Broadcast Method
To send the message to all nodes in the network, you can
send it with standard `broadcast` method, It sends packet with destination address set as
`GeneralAddressType::BROADCAST`. Every device will treat `GeneralAddressType::Broadcast`
as it's own address, will keep the message as received and will transit copy of that message further.
`main.rs`:
```
let _ = mesh_node.broadcast(
    message.into_bytes(), // Content.
    10 as LifeTimeType,   // Let message travel 10 devices before being destroyed.
);
```

### Send to exact Method
!The term `echoed message` refers to a duplicated message that has
been re-transmitted into the ether by an intermediate device.

Send to exact method - sends the message to device with exact address in the network.
The `send_to_exact` method requires the following arguments:

- `data`: A `PacketDataBytes` instance to hold the message bytes.
- `destination_device_identifier`: A `ExactAddressType` instance indicating exact target device.
- `lifetime`: A `LifeTimeType` instance to control for how far the message can travel.
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
The `receive` method optionally returns received data in a `PacketDataBytes` instance in case
if that packet was previously received by this exact device. It does not matter if that data
was sent via `broadcast`, `send_to_exact`, `ping_pong` or `send_with_transaction` method because
anyway it was sent to that exact device.
The way that packet was sent to this device can be checked in `special_state` field of returned
value. Field shall contain `PacketState` enum instance.

`main.rs`:
```
match mesh_node.receive() {
    Some(packet) => ...,
    Node => ....,
}
```

### Send Ping-Pong Method
The `send_ping_pong` method sends a message with a "ping" flag to the destination node and
waits for the same message with a "pong" flag which tells that the end device have received
the message at least once. It returns an error if the ping-pong exchange fails.
The following arguments are required:

`Ping-Pong time diagram`:
```
            +----------+              +----------+
            |  Sender  |              | Receiver |
            +--------- +              +----------+
                 |                         |     
 Ping-pong start |   --------Ping------->  |   <-- Receiver has received the message
                 |                         |     
Ping-pong finish |   <-------Pong--------  |     
                 |                         |     
                                    
```

- `data`: A `PacketDataBytes` instance.
- `destination_device_identifier`: A `ExactAddressType` instance, that indicates exact target device address.
- `lifetime`: A `LifeTimeType` instance.
- `timeout`: An `ms` instance specifying how long to wait for a response.

`main.rs`:
```
let _ = mesh_node.send_ping_pong(
    message.into_bytes(),              // Content.
    ExactAddressType::new(2).unwrap(), // Send to device with address 2.
    10 as LifeTimeType, // Let message travel 10 devices before being destroyed.
    1000 as ms,
    || {
        Instant::now()
            .duration_since(program_start_time)
            .as_millis() as ms
    },
    &mut serial,
);
```

### Send with Transaction Method
The `send_with_transaction` method sends a message and handles all further work to
ensure the target device have received it only once and correctly. It returns an error if the transaction failed.

`Transaction time diagram`:
```
                      +----------+              +----------+
                      |  Sender  |              | Receiver |
                      +--------- +              +----------+
                           |                         |     
    *Transaction start     | ---SendTransaction--->  |    \
                           |                         |     (increment packet id by 1)
                   /       | <--AcceptTransaction--  |    /
(increment packet id by 1) |                         |     
                   \       | ---InitTransaction--->  |    \ <--- Receiver has received the message
                           |                         |     (increment packet id by 1) 
    *Transaction finish    | <--FinishTransaction--  |    /                           
                           |                         |     
                                    
```

The required arguments are:
- `data`: A `PacketDataBytes` instance.
- `destination_device_identifier`: A `ExactAddressType` instance, that indicates exact target device address.
- `lifetime`: A `LifeTimeType` instance.
- `timeout`: An `ms` instance to specify the response wait time.

`main.rs`:
```
match mesh_node.send_with_transaction(
    message.into_bytes(),              // Content.
    ExactAddressType::new(2).unwrap(), // Send to device with address 2.
    10 as LifeTimeType, // Let message travel 10 devices before being destroyed.
    2000 as ms,
    || {
        Instant::now()
            .duration_since(program_start_time)
            .as_millis() as ms
    },
    &mut serial,
);
```

### Update Method
The `update` method is used to perform all internal operation of the `Node`.
It shall be called in a loop with providing embedded-serial implemented structure
and current_time im milliseconds. It allows `Node` to interact with outer world.
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
