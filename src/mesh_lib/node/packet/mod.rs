mod bitpos;
mod config;
mod traits;
mod types;

use core::slice::Iter;

pub use traits::{
    DataPacker, FromBytes, PacketFlagOps, Serializer, StateMutator, UniqueId, UniqueIdExtractor,
};

pub use config::{BROADCAST_RESERVED_IDENTIFIER, CONTENT_SIZE, PACKET_BYTES_COUNT};

use crate::{mesh_lib::node::packet::bitpos::set_flag, serial_debug};

use self::{
    bitpos::is_flag_set,
    types::{
        AddressType, ChecksumType, FlagsType, ACCEPT_TRANSACTION_FLAG, CHECKSUM_TYPE_SIZE,
        DATA_LENGTH_TYPE_SIZE, DATA_TYPE_SIZE, DEVICE_IDENTIFIER_TYPE_SIZE,
        FINISH_TRANSACTION_FLAG, FLAGS_TYPE_SIZE, ID_TYPE_SIZE, IGNORE_DUPLICATIONS_FLAG,
        INITIATE_TRANSACTION_FLAG, LIFETIME_TYPE_SIZE, PING_FLAG, PONG_FLAG, SEND_TRANSACTION_FLAG,
    },
};

pub use self::types::{IdType, LifeTimeType, PacketDataBytes, PacketSerializedBytes};

pub use super::router::SpecState;
use super::PacketMetaData;

#[derive(Clone, PartialEq, Eq)]
pub struct DeviceIdentifier(pub AddressType);

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
        spec_state: SpecState,
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

    /// Checks if the calculated checksum of the packet
    /// matches to the already stored one.
    pub fn is_checksum_correct(&self) -> bool {
        self.calculate_packet_sum() == self.checksum
    }

    pub fn get_spec_state(&self) -> SpecState {
        if self.is_ping_flag_set() {
            return SpecState::PingPacket;
        }
        if self.is_pong_flag_set() {
            return SpecState::PongPacket;
        }
        if self.is_send_transaction_flag_set() {
            return SpecState::SendTransaction;
        }
        if self.is_accept_transaction_flag_set() {
            return SpecState::AcceptTransaction;
        }
        if self.is_initiate_transaction_flag_set() {
            return SpecState::InitTransaction;
        }
        if self.is_finish_transaction_flag_set() {
            return SpecState::FinishTransaction;
        }
        SpecState::Normal
    }

    pub fn set_spec_state(&mut self, new_state: SpecState) {
        self.set_ping_flag(false);
        self.set_pong_flag(false);
        self.set_send_transaction_flag(false);
        self.set_accept_transaction_flag(false);
        self.set_initiate_transaction_flag(false);
        self.set_finish_transaction_flag(false);
        match new_state {
            SpecState::Normal => (),
            SpecState::PingPacket => {
                self.set_ping_flag(true);
            }
            SpecState::PongPacket => {
                self.set_pong_flag(true);
            }
            SpecState::SendTransaction => {
                self.set_send_transaction_flag(true);
            }
            SpecState::AcceptTransaction => {
                self.set_accept_transaction_flag(true);
            }
            SpecState::InitTransaction => {
                self.set_initiate_transaction_flag(true);
            }
            SpecState::FinishTransaction => {
                self.set_finish_transaction_flag(true);
            }
        }
    }

    /// Calculates and returns checksum of whole packet.
    ///
    /// Checksum consist of next fields:
    ///      source_device_identifier
    ///      destination_device_identifier
    ///      flags
    ///      data_length
    ///      data
    fn calculate_packet_sum(&self) -> ChecksumType {
        let result: ChecksumType = 0;

        // Calculate source_device_identifier
        let result = result.overflowing_add(self.source_device_identifier.0).0;

        // Calculate destination_device_identifier
        let mut result = result
            .overflowing_add(self.destination_device_identifier.0)
            .0;

        // Calculate id
        for byte in self.id.to_be_bytes() {
            result = result.overflowing_add(byte).0;
        }

        // Calculate lifetime
        for byte in self.lifetime.to_be_bytes() {
            result = result.overflowing_add(byte).0;
        }

        // Calculate flags
        for byte in self.flags.to_be_bytes() {
            result = result.overflowing_add(byte).0;
        }

        // Calculate data_length
        for byte in self.data_length.to_be_bytes() {
            result = result.overflowing_add(byte).0;
        }

        // Calculate data
        for byte in self.data.iter() {
            result = result.overflowing_add(*byte).0;
        }

        result
    }

    /// Calculates checksum for this packet, and sets
    /// calculated value into .checksum field. returns
    /// new summarized packet.
    pub fn summarized(mut self) -> Packet {
        self.checksum = self.calculate_packet_sum();
        self
    }
}

impl DataPacker for Packet {
    fn pack(packet_meta_data: PacketMetaData) -> Self {
        Packet::new(
            packet_meta_data.source_device_identifier,
            packet_meta_data.destination_device_identifier,
            packet_meta_data.packet_id,
            packet_meta_data.lifetime,
            packet_meta_data.spec_state,
            packet_meta_data.data,
        )
    }

    fn unpack(self: Self) -> PacketMetaData {
        PacketMetaData {
            data: self.data.iter().map(|el| *el).collect(), // Can it be simplified?
            source_device_identifier: self.source_device_identifier.clone(),
            destination_device_identifier: self.destination_device_identifier.clone(),
            lifetime: self.lifetime,
            filter_out_duplication: self.is_ignore_duplication_flag_set(),
            spec_state: self.get_spec_state(),
            packet_id: self.id,
        }
    }
}

fn deserialize_field<T, const GENERIC_TYPE_SIZE: usize>(bytes_iterator: &mut Iter<'_, u8>) -> T
where
    T: From<u8> + Default + FromBytes<GENERIC_TYPE_SIZE>,
{
    let mut field: [u8; GENERIC_TYPE_SIZE] = [0; GENERIC_TYPE_SIZE];
    for entry in field.iter_mut() {
        *entry = *bytes_iterator.next().unwrap_or_else(|| {
            serial_debug!("Could not deserialize byte of field");
            &0u8
        });
    }
    T::from_be_bytes(field)
}

impl Serializer for Packet {
    fn serialize(self) -> types::PacketSerializedBytes {
        let mut result = PacketSerializedBytes::new();
        // source_device_identifier: Deviceidentifier,
        for b in self.source_device_identifier.0.to_be_bytes() {
            result.push(b).unwrap_or_else(|_| {
                serial_debug!("Could not serialize byte of source_device_identifier field")
            });
        }

        // destination_device_identifier: Deviceidentifier,
        for b in self.destination_device_identifier.0.to_be_bytes() {
            result.push(b).unwrap_or_else(|_| {
                serial_debug!("Could not serialize byte of destination_device_identifier field")
            });
        }

        // id: IdType
        for b in self.id.to_be_bytes() {
            result
                .push(b)
                .unwrap_or_else(|_| serial_debug!("Could not serialize byte of id field"));
        }

        // lifetime: LifeTimeType
        for b in self.lifetime.to_be_bytes() {
            result
                .push(b)
                .unwrap_or_else(|_| serial_debug!("Could not serialize byte of lifetime field"));
        }

        // flags: FlagsType,
        for b in self.flags.to_be_bytes() {
            result
                .push(b)
                .unwrap_or_else(|_| serial_debug!("Could not serialize byte of flags field"));
        }

        // data_length: usize,
        for b in self.data_length.to_be_bytes() {
            result
                .push(b)
                .unwrap_or_else(|_| serial_debug!("Could not serialize byte of data_length field"));
        }

        // data: PacketDataBytes,
        for b in self.data {
            result
                .push(b)
                .unwrap_or_else(|_| serial_debug!("Could not serialize byte of data field"));
        }

        // checksum: ChecksumType,
        for b in self.checksum.to_be_bytes() {
            result
                .push(b)
                .unwrap_or_else(|_| serial_debug!("Could not serialize byte of checksum field"));
        }
        result
    }

    fn deserialize(bytes: types::PacketSerializedBytes) -> Self {
        let mut bytes_iterator = bytes.iter();

        let source_device_identifier =
            deserialize_field::<AddressType, DEVICE_IDENTIFIER_TYPE_SIZE>(&mut bytes_iterator);
        let source_device_identifier = DeviceIdentifier(source_device_identifier);

        let destination_device_identifier =
            deserialize_field::<AddressType, DEVICE_IDENTIFIER_TYPE_SIZE>(&mut bytes_iterator);
        let destination_device_identifier = DeviceIdentifier(destination_device_identifier);

        let id = deserialize_field::<IdType, ID_TYPE_SIZE>(&mut bytes_iterator);
        let lifetime = deserialize_field::<LifeTimeType, LIFETIME_TYPE_SIZE>(&mut bytes_iterator);
        let flags = deserialize_field::<FlagsType, FLAGS_TYPE_SIZE>(&mut bytes_iterator);
        let data_length = deserialize_field::<usize, DATA_LENGTH_TYPE_SIZE>(&mut bytes_iterator);

        // data: PacketDataBytes, // Is vector of bytes.
        let mut data: PacketDataBytes = PacketDataBytes::new();
        for _ in 0..DATA_TYPE_SIZE {
            data.push(*bytes_iterator.next().unwrap_or_else(|| {
                serial_debug!("Could not take byte for deserialization of data");
                &0u8
            }))
            .unwrap_or_else(|_| {
                serial_debug!("Could not push byte of serialized data");
            });
        }
        let checksum = deserialize_field::<ChecksumType, CHECKSUM_TYPE_SIZE>(&mut bytes_iterator);
        Packet {
            source_device_identifier,
            destination_device_identifier,
            id,
            lifetime,
            flags,
            data_length,
            data,
            checksum,
        }
    }
}

impl UniqueIdExtractor for Packet {
    fn get_unique_id(&self) -> UniqueId {
        UniqueId::new(self.source_device_identifier.clone(), self.id)
    }
}

impl PacketFlagOps for Packet {
    // IGNORE_DUPLICATIONS_FLAG
    fn set_ignore_duplication_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, IGNORE_DUPLICATIONS_FLAG, new_state);
    }

    fn is_ignore_duplication_flag_set(&self) -> bool {
        is_flag_set(self.flags, IGNORE_DUPLICATIONS_FLAG)
    }

    // PING_FLAG
    fn set_ping_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, PING_FLAG, new_state);
    }

    fn is_ping_flag_set(&self) -> bool {
        is_flag_set(self.flags, PING_FLAG)
    }

    // PONG_FLAG
    fn set_pong_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, PONG_FLAG, new_state);
    }
    fn is_pong_flag_set(&self) -> bool {
        is_flag_set(self.flags, PONG_FLAG)
    }

    // TRANSACTION_SEND_FLAG
    fn set_send_transaction_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, SEND_TRANSACTION_FLAG, new_state);
    }
    fn is_send_transaction_flag_set(&self) -> bool {
        is_flag_set(self.flags, SEND_TRANSACTION_FLAG)
    }

    // ACCEPT_TRANSACTION_FLAG
    fn set_accept_transaction_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, ACCEPT_TRANSACTION_FLAG, new_state);
    }
    fn is_accept_transaction_flag_set(&self) -> bool {
        is_flag_set(self.flags, ACCEPT_TRANSACTION_FLAG)
    }

    // INITIATE_TRANSACTION_FLAG
    fn set_initiate_transaction_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, INITIATE_TRANSACTION_FLAG, new_state);
    }
    fn is_initiate_transaction_flag_set(&self) -> bool {
        is_flag_set(self.flags, INITIATE_TRANSACTION_FLAG)
    }

    // FINISH_TRANSACTION_FLAG
    fn set_finish_transaction_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, FINISH_TRANSACTION_FLAG, new_state);
    }
    fn is_finish_transaction_flag_set(&self) -> bool {
        is_flag_set(self.flags, FINISH_TRANSACTION_FLAG)
    }
}
