//! Lightweight mesh communication protocol for embedded devices.
//!
//! `embedded-nano-mesh` allows you to build a **self-healing mesh network**
//! using very cheap microcontrollers and simple serial radio modules.
//!
//! The library is `no_std`, non-blocking and works with any serial interface
//! that implements [`embedded_io::Read`], [`embedded_io::Write`] and
//! [`embedded_io::ReadReady`].
//!
//! # Quick start
//!
//! ```
//! use embedded_nano_mesh::{ms, ExactAddressType, LifeTimeType, Node, NodeConfig, NodeString};
//!
//! let mut mesh_node = Node::new(NodeConfig {
//!     device_address: ExactAddressType::new(1).unwrap(),
//!     listen_period: 150 as ms,
//! });
//!
//! let _ = mesh_node.send_to_exact(
//!     NodeString::from_iter("Hello".chars()).into_bytes(),
//!     ExactAddressType::new(2).unwrap(),
//!     10 as LifeTimeType,
//!     true,
//! );
//!
//! // In the main loop:
//! // mesh_node.update(&mut serial, current_time);
//! // if let Some(packet) = mesh_node.receive() { ... }
//! ```
//!
//! Delivery guarantees (transactions, ping-pong, acknowledgements, retries)
//! are intentionally **not** part of the protocol core. They belong to the
//! application layer and can be built on top of `send_to_exact` / `broadcast` /
//! `receive`.
#![no_std]
mod mesh_lib;

pub use mesh_lib::*;
