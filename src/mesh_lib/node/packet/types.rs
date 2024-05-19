use super::constants::{
    ADDRESS_TYPE_SIZE, CONTENT_SIZE, DATA_LENGTH_TYPE_SIZE, MULTICAST_RESERVED_IDENTIFIER,
    PACKET_BYTES_SIZE,
};

use heapless::Vec;

use super::FromBytes;

/// Type alias for packet identification number.
pub type IdType = u8;

/// Type alias for packet bit flags.
pub type FlagsType = u8;

pub type ExactDeviceAddressType = core::num::NonZeroU8;

/// Type to strict interaction with addressing during use of the library.
pub enum GeneralAddressType {
    /// Sends the packet to exact device with this address.
    Exact(ExactDeviceAddressType),

    /// Sends the packet to all devices it can reach.
    Multicast,
}

impl Into<AddressType> for GeneralAddressType {
    fn into(self) -> AddressType {
        match self {
            Self::Exact(address) => address.get(),
            Self::Multicast => MULTICAST_RESERVED_IDENTIFIER,
        }
    }
}

impl From<AddressType> for GeneralAddressType {
    fn from(address: AddressType) -> Self {
        match core::num::NonZeroU8::new(address) {
            Some(address) => Self::Exact(address),
            None => Self::Multicast,
        }
    }
}

/// Type alias for packet address identification number.
pub type AddressType = u8;

/// Type alias for packet checksum.
pub type ChecksumType = u8;

/// Type alias for packet data length.
pub type DataLengthType = u16;

/// Type alias for packet lifetime. This value contains the information,
/// about for how many times the packet can be re-sent.
/// It has sense to contain same capacity of possible values same
/// as `AddressType` - in order to make the packet possible
/// to pass all the nodes of the network.
pub type LifeTimeType = AddressType;

/// Type alias for data contained in the packet.
pub type PacketDataBytes = Vec<u8, { CONTENT_SIZE }>;

/// Type alias that represents serialized packet bytes sequence.
pub type PacketSerializedBytes = Vec<u8, { PACKET_BYTES_SIZE }>;

impl FromBytes<ADDRESS_TYPE_SIZE> for AddressType {
    fn from_be_bytes(bytes: [u8; ADDRESS_TYPE_SIZE]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

impl FromBytes<DATA_LENGTH_TYPE_SIZE> for DataLengthType {
    fn from_be_bytes(bytes: [u8; DATA_LENGTH_TYPE_SIZE]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

/// State of the packet.
#[derive(PartialEq, Eq, Clone)]
pub enum PacketState {
    Normal,
    Ping,
    Pong,
    SendTransaction,
    AcceptTransaction,
    InitTransaction,
    FinishTransaction,
}
