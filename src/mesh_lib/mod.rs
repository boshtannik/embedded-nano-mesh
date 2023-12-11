pub mod millis;
pub mod node;
pub mod serial;

pub use millis::ms;
pub use node::{AddressType, LifeTimeType, Node, NodeError, NodeString, PacketMetaData};
pub use serial::Usart;

use self::millis::PlatformTime;

/// This structure is made to group the required settings,
/// which are need to be provided to let mesh `Node` to be
/// functionally working.
pub struct NodeConfig<'t, TIMER>
where
    TIMER: PlatformTime,
{
    /// This is the identifier, that representds the device within the network.
    /// It is made in `AddressType` type in order to simplify usage and reading
    /// of the value.
    pub device_identifier: AddressType,

    /// The period, during which the `Node` will be in listen only mode.
    /// This prevents the `Node` from constantly speaking into the ether
    /// in order to reduce the ethter being jammed by immediate answers
    /// from `Nodess, that have just received the messages.
    pub listen_period: ms,

    // It is the `Usart` interface, which `Node` is using to send packed
    // messages and receive packed messages.
    // It is expected that the modules of radio transmitter and receiver
    // will be connected to the current MCU to provide communication
    // between `Node`s
    pub usart: Usart,

    // Reference to structure that implements PlatformTime trait.
    // It is used by the `Node` to calculate internal packet statusses, sending, duplication
    // timings, etc..
    pub millis_timer: &'t TIMER,
}

/// Receives `NodeConfig` instance as instance,
/// which helds configurations of current node device.
/// Makes all necessarry preparations, and returns `Node`
/// instance.
pub fn init_node<'t, TIMER>(config: NodeConfig<TIMER>) -> Node<TIMER>
where
    TIMER: PlatformTime,
{
    serial::init(config.usart);
    Node::new(
        config.device_identifier,
        config.listen_period,
        config.millis_timer,
    )
}
