# embedded-nano-mesh

A tiny mesh networking protocol for small embedded devices.

`embedded-nano-mesh` allows you to build a **self-healing mesh network**
using **very cheap microcontrollers and simple serial radio modules**.

It is designed to run on extremely small devices such as:

- Arduino Nano / Uno / Mini
- small STM32 boards
- ESP8266 / ESP32
- Raspberry Pi Pico
- Linux devices (Raspberry Pi, Orange Pi, PCs)

The only thing required is a **serial connection to a radio module**.

No WiFi stack.  
No heavy networking libraries.  
Just a lightweight mesh protocol.

------------------------------------------------------------------------

# Why this library exists

Many mesh solutions require:

- powerful hardware
- complex configuration
- large memory footprint

`embedded-nano-mesh` is different.

It was built with the goal of running on **very small and cheap
boards**.

You can create a mesh network using:

    Microcontroller + Serial Radio Module

Example:

    +--------------+       +--------------+
    | Arduino Nano |       | HC-12        |
    | STM32        |   +   | JDY-41       |     = Mesh node
    | ESP8266      |       | LoRa module  |
    | Linux PC     |       | etc.         |
    +--------------+       +--------------+



Connect multiple nodes together and they will **automatically form a
mesh network**.

------------------------------------------------------------------------

# What you can build

This protocol works well for small distributed systems:

- Home automation
- Agricultural sensor networks
- Remote telemetry
- Industrial data collection
- Device-to-device messaging
- Remote control systems

Because nodes forward messages to each other, the **network
automatically extends its range**.

------------------------------------------------------------------------

# Key features

- `no_std` compatible
- Works on **very low-memory devices**
- Runs on **Linux and microcontrollers**
- Works with **any serial radio module**
- Self-healing mesh network
- Configurable packet lifetime (hop limit)
- Duplicate packet filtering
- Very small protocol overhead
- Easy to port to new hardware

------------------------------------------------------------------------

# Quick start (5 minutes)

Add the library to your `Cargo.toml`:

    embedded-nano-mesh = "2.1.10"

Then provide a serial interface using `embedded-io`.

Example for Linux:

    embedded-nano-mesh-linux-io = "0.0.1"

Example usage:

``` rust
let mut interface = LinuxIO::new(...);
let mut mesh_node = ...;

mesh_node.send_to_exact(
    message.into_bytes(),
    ExactAddressType::new(2).unwrap(),
    10 as LifeTimeType,
    true,  // Ignore duplicated packets
).unwrap();

loop {
    mesh_node.update(&mut interface, current_time);
}
```

That’s it.

Your device is now part of a mesh network.

------------------------------------------------------------------------

# How routing works

The protocol intentionally uses a **very simple routing model**.

When a node sends a packet:

1.  Nearby nodes receive it
2.  Each node decides whether to forward it
3.  The packet spreads through the network

This behaves similar to **ripples in water**.

To prevent infinite spreading, packets contain a **lifetime (hop
limit)**.

Each hop decreases the lifetime.

Example:

    lifetime = 10

The packet can travel **up to 10 nodes** before it disappears.

Examples:

    lifetime = 1   → only nearest devices
    lifetime = 5   → medium network range
    lifetime = 10  → large mesh

------------------------------------------------------------------------

# Avoiding duplicate packets

Mesh networks often suffer from packet echo.

`embedded-nano-mesh` prevents this using a **duplicate filter**.

Each node remembers recently seen packets:

    (sender_id, packet_id)

If the same packet appears again within a short time window, it is
automatically ignored.

This keeps the network from being flooded by echoes.

------------------------------------------------------------------------

# Examples

Example projects:

Arduino Nano example:  
https://github.com/boshtannik/embedded-nano-mesh-arduino-nano-example

Linux example:  
https://github.com/boshtannik/embedded-nano-mesh-linux-example

CLI tool:  
https://github.com/boshtannik/embedded-nano-mesh-cli-tool

------------------------------------------------------------------------

# Platform support

Currently tested on:

- Arduino Nano
- Linux (PC / Raspberry Pi)

Expected to work on:

- STM32
- ESP8266
- ESP32
- Raspberry Pi Pico
- Windows
- macOS

Porting to new platforms is easy.

------------------------------------------------------------------------

# Porting to new hardware

The protocol communicates with radios through the `embedded-io` trait.

To port the library to a new platform you only need to implement:

    embedded_io::Read
    embedded_io::Write

for your serial interface.

That’s all.

Once your serial driver implements `embedded-io`, the mesh protocol will
work.

------------------------------------------------------------------------

# Managing packet collisions

To reduce packet collisions, it is recommended to use different
`listen_period` values for different nodes.

Example:

    device 1 → 230 ms
    device 2 → 240 ms
    device 3 → 250 ms

This prevents devices from synchronizing their transmissions.

------------------------------------------------------------------------

# Protocol limits (default configuration)

- Payload size: **32 bytes**
- Max addresses: **255**
- Receive queue: **5 packets**
- Transit queue: **5 packets**
- Duplicate filter size: **8 packets**
- Duplicate ignore period: **1000 ms**

These values can be adjusted if needed.

------------------------------------------------------------------------

# Security

The protocol **does not implement encryption**.

If encryption is required, it should be implemented in the application
layer.

------------------------------------------------------------------------

# Status

Current version: **2.1.10**

Features:

- sending messages
- receiving messages
- hop-limited routing
- broadcast
- duplicate filtering
- ping-pong communication
- transactions
- intermediate node forwarding

------------------------------------------------------------------------

# Community

Telegram channel:

https://t.me/embedded_nano_mesh

GitHub:

https://github.com/boshtannik

------------------------------------------------------------------------

# Support the project

If this project helped you, you can support development.

Bitcoin:

    bc1qc50tm0ppj3hh7fecd6d0rv8tdygy8uhe2cemzt

Buy me a coffee:

https://www.buymeacoffee.com/boshtannik

------------------------------------------------------------------------

# License

This project is licensed under:

- MIT
- Apache 2.0
- GPLv3

You may choose the license that best fits your needs.

------------------------------------------------------------------------

# Contributing

Contributions are welcome.

Please include in your pull request:

- Issue reference
- Short explanation of the fix
- Description of the solution
