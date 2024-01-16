mod bitpos;
mod constants;
mod traits;
mod types;

pub mod implementations;
pub mod meta_data;
pub mod trait_implementations;

pub use meta_data::{PacketMetaData, PacketMetaDataError};

pub use traits::{
    DataPacker, FromBytes, PacketFlagOps, Serializer, StateMutator, UniqueId, UniqueIdExtractor,
};

pub use constants::{
    ADDRESS_TYPE_SIZE, CHECKSUM_TYPE_SIZE, CONTENT_SIZE, DATA_LENGTH_TYPE_SIZE, FLAGS_TYPE_SIZE,
    ID_TYPE_SIZE, LIFETIME_TYPE_SIZE, MULTICAST_RESERVED_IDENTIFIER, PACKET_BYTES_COUNT,
};

use self::types::{ChecksumType, DataLengthType, FlagsType};

pub use self::types::{AddressType, IdType, LifeTimeType, PacketDataBytes, PacketSerializedBytes};

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
    fn new(
        source_device_identifier: AddressType,
        destination_device_identifier: AddressType,
        id: IdType,
        lifetime: LifeTimeType,
        spec_state: PacketState,
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
        new_packet.set_spec_state(spec_state);
        new_packet.summarized()
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
