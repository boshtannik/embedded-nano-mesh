use super::constants::{CONTENT_SIZE, PACKET_BYTES_SIZE};

use heapless::Vec;

use super::FromBytes;

/// Type alias for packet identification number.
pub type IdType = u8;

/// Type alias for packet bit flags.
pub type FlagsType = u8;

/// Type alias for device address identification number.
/// It can contain only non-zero positive number.
/// The zero value is reserved for broadcast address.
#[cfg(enm_address_type_u8)]
pub type ExactAddressType = core::num::NonZeroU8;
#[cfg(enm_address_type_u16)]
pub type ExactAddressType = core::num::NonZeroU16;
#[cfg(enm_address_type_u32)]
pub type ExactAddressType = core::num::NonZeroU32;
/// Type to strict interaction with addressing during use of the library.
/// It provides options to send packet to exact device or to all devices it can reach.
#[derive(Eq, PartialEq, Clone)]
pub enum GeneralAddressType {
    /// Sends the packet to exact device with this address.
    Exact(ExactAddressType),

    /// Sends the packet to all devices it can reach.
    Broadcast,
}

impl Into<GeneralAddressType> for ExactAddressType {
    fn into(self) -> GeneralAddressType {
        GeneralAddressType::Exact(self)
    }
}

impl Into<AddressType> for GeneralAddressType {
    fn into(self) -> AddressType {
        match self {
            Self::Exact(address) => address.get(),
            Self::Broadcast => 0 as AddressType,
        }
    }
}

impl From<AddressType> for GeneralAddressType {
    fn from(address: AddressType) -> Self {
        match ExactAddressType::new(address) {
            Some(address) => Self::Exact(address),
            None => Self::Broadcast,
        }
    }
}

/// Type alias for packet address identification number.
#[cfg(enm_address_type_u8)]
pub type AddressType = u8;
#[cfg(enm_address_type_u16)]
pub type AddressType = u16;
#[cfg(enm_address_type_u32)]
pub type AddressType = u32;

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

/// Type alias for bytes of data contained in the packet.
pub type PacketDataBytes = Vec<u8, { CONTENT_SIZE }>;

/// Type alias that represents serialized packet bytes sequence.
pub type PacketSerializedBytes = Vec<u8, { PACKET_BYTES_SIZE }>;

impl FromBytes<1> for u8 {
    fn from_be_bytes(bytes: [u8; 1]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

impl FromBytes<2> for u16 {
    fn from_be_bytes(bytes: [u8; 2]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

impl FromBytes<4> for u32 {
    fn from_be_bytes(bytes: [u8; 4]) -> Self {
        Self::from_be_bytes(bytes)
    }
}
