mod node;

pub use node::{
    AddressType, LifeTimeType, Node, NodeConfig, NodeString, NodeUpdateError, SendError,
    SpecialSendError, MULTICAST_RESERVED_IDENTIFIER,
};
pub use platform_millis::ms;
