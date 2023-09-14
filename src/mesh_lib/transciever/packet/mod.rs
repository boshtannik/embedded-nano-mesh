mod config;
mod traits;
mod types;

use core::slice::Iter;

pub use traits::{DataPacker, FromBytes, Serializer, UniqueId, UniqueIdExtractor};

pub use config::{CONTENT_SIZE, PACKET_BYTES_COUNT};

use crate::serial_debug;

use self::types::{
    AddressType, ChecksumType, FlagsType, CHECKSUM_TYPE_SIZE, DATA_LENGTH_TYPE_SIZE,
    DATA_TYPE_SIZE, DEVICE_IDENTIFYER_TYPE_SIZE, FLAGS_TYPE_SIZE, ID_TYPE_SIZE, LIFETIME_TYPE_SIZE,
};

pub use self::types::{IdType, LifeTimeType, PacketDataBytes, PacketSerializedBytes};

#[derive(Clone, PartialEq, Eq)]
pub struct DeviceIdentifyer(pub AddressType);

#[derive(Clone)]
pub struct Packet {
    source_device_identifyer: DeviceIdentifyer,
    destination_device_identifyer: DeviceIdentifyer,
    id: IdType,
    lifetime: LifeTimeType,
    flags: FlagsType,
    data_length: usize,
    data: PacketDataBytes,
    checksum: ChecksumType,
}

pub enum PacketError {
    PacketLifetimeEnded,
}

impl Packet {
    fn new(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        id: IdType,
        lifetime: LifeTimeType,
        data: PacketDataBytes,
    ) -> Packet {
        let mut new_packet = Packet {
            source_device_identifyer,
            destination_device_identifyer,
            id,
            lifetime,
            flags: FlagsType::MIN,
            data_length: data.len(),
            data,
            checksum: ChecksumType::MIN,
        };
        new_packet.summarize();
        new_packet
    }

    pub const fn size_of_bytes() -> usize {
        DEVICE_IDENTIFYER_TYPE_SIZE  // source_device_identifyer
            + DEVICE_IDENTIFYER_TYPE_SIZE  // destination_device_identifyer
            + ID_TYPE_SIZE
            + LIFETIME_TYPE_SIZE
            + FLAGS_TYPE_SIZE
            + DATA_LENGTH_TYPE_SIZE
            + CONTENT_SIZE
            + CHECKSUM_TYPE_SIZE
    }

    pub fn deacrease_lifetime(mut self) -> Result<Self, PacketError> {
        match self.lifetime.cmp(&1) {
            core::cmp::Ordering::Greater => {
                self.lifetime -= 1;
                self.summarize();
                Ok(self)
            }
            _ => Err(PacketError::PacketLifetimeEnded),
        }
    }

    /// Checks if the calculated checksum of the packet
    /// matches to the already stored one.
    pub fn is_checksum_correct(&self) -> bool {
        self.calculate_packet_sum() == self.checksum
    }

    pub fn is_destination_identifyer_reached(&self, identifyer: &DeviceIdentifyer) -> bool {
        self.destination_device_identifyer == *identifyer
    }

    /// Calculates and returns checksum of whole packet.
    ///
    /// Checksum consist of next fields:
    ///      source_device_identifyer
    ///      destination_device_identifyer
    ///      flags
    ///      data_length
    ///      data
    fn calculate_packet_sum(&self) -> ChecksumType {
        let result: ChecksumType = 0;

        // Calculate source_device_identifyer
        let result = result.overflowing_add(self.source_device_identifyer.0).0;

        // Calculate destination_device_identifyer
        let mut result = result
            .overflowing_add(self.destination_device_identifyer.0)
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
    /// calculated value into .checksum field
    fn summarize(&mut self) {
        self.checksum = self.calculate_packet_sum();
    }
}

impl DataPacker for Packet {
    fn pack(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        id: IdType,
        lifetime: LifeTimeType,
        data: PacketDataBytes,
    ) -> Self {
        Packet::new(
            source_device_identifyer,
            destination_device_identifyer,
            id,
            lifetime,
            data,
        )
    }

    fn unpack(self: Self) -> PacketDataBytes {
        self.data
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
        // source_device_identifyer: DeviceIdentifyer,
        for b in self.source_device_identifyer.0.to_be_bytes() {
            result.push(b).unwrap_or_else(|_| {
                serial_debug!("Could not serialize byte of source_device_identifyer field")
            });
        }

        // destination_device_identifyer: DeviceIdentifyer,
        for b in self.destination_device_identifyer.0.to_be_bytes() {
            result.push(b).unwrap_or_else(|_| {
                serial_debug!("Could not serialize byte of destination_device_identifyer field")
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

        let source_device_identifyer =
            deserialize_field::<AddressType, DEVICE_IDENTIFYER_TYPE_SIZE>(&mut bytes_iterator);

        let destination_device_identifyer =
            deserialize_field::<AddressType, DEVICE_IDENTIFYER_TYPE_SIZE>(&mut bytes_iterator);

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
            source_device_identifyer: DeviceIdentifyer(source_device_identifyer),
            destination_device_identifyer: DeviceIdentifyer(destination_device_identifyer),
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
        UniqueId::new(self.source_device_identifyer.clone(), self.id)
    }
}
