use super::constants::{ADDRESS_TYPE_SIZE, CONTENT_SIZE, DATA_LENGTH_TYPE_SIZE, PACKET_BYTES_SIZE};

use heapless::Vec;

use super::FromBytes;

/// Type alias for packet identification number.
pub type IdType = u8;

/// Type alias for packet bit flags.
pub type FlagsType = u8;

/// Type alias for device address identification number.
/// It can contain only non-zero positive number.
/// The zero value is reserved for broadcast address.
pub type ExactAddressType = core::num::NonZeroU8;

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
        match core::num::NonZeroU8::new(address) {
            Some(address) => Self::Exact(address),
            None => Self::Broadcast,
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

/// Type alias for bytes of data contained in the packet.
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
    /// End-receiver of packet with this state - just receives the data.
    /// does no additional logic over it.
    Normal,

    /// Packet with this state being set - forces end-receiver device,
    /// to automatically respond packet with same content back with
    /// `Pong` state being set. Also receiving device receives content
    /// of the packet.
    Ping,

    /// Packet with this state being set - indicates, that receiver
    /// has successfully processed the packet.
    Pong,

    /// Packet with this state being set - forces end-receiver device,
    /// to get ready to do full transaction.
    SendTransaction,

    /// Packet with this state being set - is sent by receiver and
    /// informs sender device, that receiver have the
    /// transaction being started.
    AcceptTransaction,

    /// Packet with this state being set - is sent by sender and
    /// forces end-receiver device, to do the transaction finish.
    InitTransaction,

    /// Packet with this state being set - is sent by receiver and
    /// informs sender device, that receiver have the transaction finished.
    FinishTransaction,
}
