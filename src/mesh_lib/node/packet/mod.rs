mod bitpos;
mod constants;
mod types;

pub mod implementations;

pub mod trait_implementations;
pub mod traits;

pub use implementations::{PacketLifetimeEnded, RespondToBroadcastAddressError};

pub use traits::{
    FromBytes, PacketFlagOps, PacketUniqueId, Serializer, StateMutator, UniqueIdExtractor,
};

pub use constants::{
    ADDRESS_TYPE_SIZE, CHECKSUM_TYPE_SIZE, CONTENT_SIZE, DATA_LENGTH_TYPE_SIZE, FLAGS_TYPE_SIZE,
    ID_TYPE_SIZE, LIFETIME_TYPE_SIZE, PACKET_BYTES_SIZE,
};

use self::types::{ChecksumType, DataLengthType, FlagsType};

pub use self::types::{
    AddressType, ExactAddressType, GeneralAddressType, IdType, LifeTimeType, PacketDataBytes,
    PacketSerializedBytes,
};

pub use types::PacketState;

#[derive(Clone)]
pub struct Packet {
    source_device_identifier: AddressType,
    destination_device_identifier: AddressType,
    id: IdType,
    lifetime: LifeTimeType,
    flags: FlagsType,
    data_length: DataLengthType,
    data: PacketDataBytes,
    checksum: ChecksumType,
}

impl Packet {
    pub fn new(
        source_device_identifier: AddressType,
        destination_device_identifier: AddressType,
        id: IdType,
        lifetime: LifeTimeType,
        spec_state: PacketState,
        ignore_duplications_flag: bool,
        mut data: PacketDataBytes,
    ) -> Packet {
        while !data.is_full() {
            data.push(b'\0').unwrap_or_else(|_| ());
        }
        let mut new_packet = Packet {
            source_device_identifier,
            destination_device_identifier,
            id,
            lifetime,
            flags: FlagsType::MIN,
            data_length: data.len() as DataLengthType,
            data,
            checksum: ChecksumType::MIN,
        };
        new_packet.set_ignore_duplication_flag(ignore_duplications_flag);
        new_packet.set_spec_state(spec_state);
        new_packet
    }

    // Because some one can form and transmit packet that is marked
    // to be sent from broadcast address for example, which should
    // not be possible case. This method helps prevent it.
    pub fn has_correct_source_device_identifier(&self) -> bool {
        ExactAddressType::new(self.source_device_identifier).is_some()
    }

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
