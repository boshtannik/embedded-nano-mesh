mod bitpos;
mod constants;
mod types;

pub mod implementations;

pub mod trait_implementations;
pub mod traits;

pub use implementations::PacketLifetimeEnded;

pub use traits::{FromBytes, PacketFlagOps, PacketUniqueId, Serializer, UniqueIdExtractor};

pub use constants::{
    ADDRESS_TYPE_SIZE, CHECKSUM_TYPE_SIZE, CONTENT_SIZE, DATA_LENGTH_TYPE_SIZE, FLAGS_TYPE_SIZE,
    ID_TYPE_SIZE, LIFETIME_TYPE_SIZE, PACKET_BYTES_SIZE,
};

use self::types::{ChecksumType, DataLengthType, FlagsType};

pub use self::types::{
    AddressType, ExactAddressType, GeneralAddressType, IdType, LifeTimeType, PacketDataBytes,
    PacketSerializedBytes,
};

/// A packet that is transmitted over the mesh network.
///
/// The packet has a fixed wire format of 40 bytes:
/// source address, destination address, id, lifetime, flags,
/// data length, 32 bytes of data and a checksum.
#[derive(Clone, Debug)]
pub struct Packet {
    /// Address of the device that sent the packet.
    pub source_device_identifier: AddressType,
    destination_device_identifier: AddressType,
    /// Packet identification number assigned by the sender.
    pub id: IdType,
    lifetime: LifeTimeType,
    flags: FlagsType,
    data_length: DataLengthType,
    /// Payload of the packet, zero-padded up to `CONTENT_SIZE` bytes.
    pub data: PacketDataBytes,
    checksum: ChecksumType,
}

impl Packet {
    /// Creates a new packet with the given addresses, id, lifetime and data.
    ///
    /// `ignore_duplications_flag` tells other nodes whether they shall
    /// filter echoes of this packet. The payload is zero-padded up to
    /// `CONTENT_SIZE` bytes; the real length is stored in `data_length`.
    pub fn new(
        source_device_identifier: AddressType,
        destination_device_identifier: AddressType,
        id: IdType,
        lifetime: LifeTimeType,
        ignore_duplications_flag: bool,
        mut data: PacketDataBytes,
    ) -> Packet {
        let data_length = data.len() as DataLengthType;
        while !data.is_full() {
            data.push(b'\0').unwrap_or_else(|_| ());
        }
        let mut new_packet = Packet {
            source_device_identifier,
            destination_device_identifier,
            id,
            lifetime,
            flags: FlagsType::MIN,
            data_length,
            data,
            checksum: ChecksumType::MIN,
        };
        new_packet.set_ignore_duplication_flag(ignore_duplications_flag);
        new_packet
    }

    /// Checks that the packet was sent from a valid (non-zero) address.
    /// This prevents handling of packets that pretend to be sent from
    /// the broadcast address, which must not happen in a real network.
    pub fn has_correct_source_device_identifier(&self) -> bool {
        ExactAddressType::new(self.source_device_identifier).is_some()
    }

    /// Returns the size of a serialized packet in bytes (40).
    pub const fn size_of_bytes() -> usize {
        ADDRESS_TYPE_SIZE               // source_device_identifier
        + ADDRESS_TYPE_SIZE             // destination_device_identifier
        + ID_TYPE_SIZE
        + LIFETIME_TYPE_SIZE
        + FLAGS_TYPE_SIZE
        + DATA_LENGTH_TYPE_SIZE
        + CONTENT_SIZE
        + CHECKSUM_TYPE_SIZE
    }
}
