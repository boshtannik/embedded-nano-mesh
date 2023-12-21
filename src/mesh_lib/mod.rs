pub mod node;
use platform_millis::ms;

pub use node::{AddressType, LifeTimeType, Node, NodeString};

/// This structure is made to group the required settings,
/// which are need to be provided to let mesh `Node` to be
/// functionally working.
pub struct NodeConfig {
    /// This is the identifier, that representds the device within the network.
    /// It is made in `AddressType` type in order to simplify usage and reading
    /// of the value.
    pub device_identifier: AddressType,

    /// The period, during which the `Node` will be in listen only mode.
    /// This prevents the `Node` from constantly speaking into the ether
    /// in order to reduce the ethter being jammed by immediate answers
    /// from `Node`s, that have just received the messages.
    pub listen_period: ms,
}

/// Receives `NodeConfig` instance as instance,
/// which helds configurations of current node device.
/// Makes all necessarry preparations, and returns `Node`
/// instance.
pub fn init_node(config: NodeConfig) -> Node {
    Node::new(config.device_identifier, config.listen_period)
}
