pub mod millis;
pub mod node;
pub mod serial;

// TODO: Allow set dynamically, or conditionally depending on target architecture.
use avr_device::atmega328p::TC0;

pub use millis::{millis, millis_init, ms};
pub use node::{DeviceIdentifier, LifeTimeType, Node, NodeError, NodeString, PacketMetaData};
pub use serial::Usart;

pub struct NodeConfig {
    pub device_identifier: DeviceIdentifier,
    pub listen_period: ms,
    pub usart: Usart,
    pub millis_timer: TC0,
}

pub fn init_node(config: NodeConfig) -> Node {
    millis_init(config.millis_timer);
    serial::init(config.usart);
    unsafe { avr_device::interrupt::enable() };
    Node::new(config.device_identifier, config.listen_period)
}
