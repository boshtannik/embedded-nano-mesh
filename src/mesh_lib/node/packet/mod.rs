mod bitpos;
mod config;
mod traits;
mod types;

pub mod implementations;
pub mod trait_implementations;

pub use traits::{
    DataPacker, FromBytes, PacketFlagOps, Serializer, StateMutator, UniqueId, UniqueIdExtractor,
};

pub use config::{BROADCAST_RESERVED_IDENTIFIER, CONTENT_SIZE, PACKET_BYTES_COUNT};

use self::types::{
    ChecksumType, FlagsType, CHECKSUM_TYPE_SIZE, DATA_LENGTH_TYPE_SIZE,
    DEVICE_IDENTIFIER_TYPE_SIZE, FLAGS_TYPE_SIZE, ID_TYPE_SIZE, LIFETIME_TYPE_SIZE,
};

pub use self::types::{
    DeviceIdentifier, IdType, LifeTimeType, PacketDataBytes, PacketSerializedBytes,
};

use super::PacketMetaData;
pub use types::PacketState;

#[derive(Clone)]
pub struct Packet {
    source_device_identifier: DeviceIdentifier,
    destination_device_identifier: DeviceIdentifier,
    id: IdType,
    lifetime: LifeTimeType,
    flags: FlagsType,
    data_length: usize,
    data: PacketDataBytes,
    checksum: ChecksumType,
}

impl Packet {
    fn new(
        source_device_identifier: DeviceIdentifier,
        destination_device_identifier: DeviceIdentifier,
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
            data_length: data.len(),
            data,
            checksum: ChecksumType::MIN,
        };
        new_packet.set_spec_state(spec_state);
        new_packet.summarized()
    }

    pub const fn size_of_bytes() -> usize {
        DEVICE_IDENTIFIER_TYPE_SIZE  // source_device_identifier
        + DEVICE_IDENTIFIER_TYPE_SIZE  // destination_device_identifier
        + ID_TYPE_SIZE
        + LIFETIME_TYPE_SIZE
        + FLAGS_TYPE_SIZE
        + DATA_LENGTH_TYPE_SIZE
        + CONTENT_SIZE
        + CHECKSUM_TYPE_SIZE
    }
}
