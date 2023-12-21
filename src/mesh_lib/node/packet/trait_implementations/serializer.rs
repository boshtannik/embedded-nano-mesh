use super::super::traits::Serializer;
use super::super::Packet;

use super::super::constants::{
    ADDRESS_TYPE_SIZE, CHECKSUM_TYPE_SIZE, DATA_LENGTH_TYPE_SIZE, DATA_TYPE_SIZE, FLAGS_TYPE_SIZE,
    ID_TYPE_SIZE, LIFETIME_TYPE_SIZE,
};

use super::super::types::{
    AddressType, ChecksumType, FlagsType, IdType, LifeTimeType, PacketDataBytes,
    PacketSerializedBytes,
};

use platform_serial_arduino_nano::serial_debug;

use super::super::traits::FromBytes;
use core::slice::Iter;

impl Serializer for Packet {
    fn serialize(self) -> PacketSerializedBytes {
        let mut result = PacketSerializedBytes::new();
        // source_device_identifier: Deviceidentifier,
        for b in self.source_device_identifier.to_be_bytes() {
            result.push(b).unwrap_or_else(|_| {
                serial_debug!("Could not serialize byte of source_device_identifier field")
            });
        }

        // destination_device_identifier: Deviceidentifier,
        for b in self.destination_device_identifier.to_be_bytes() {
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

    fn deserialize(bytes: PacketSerializedBytes) -> Self {
        let mut bytes_iterator = bytes.iter();

        let source_device_identifier =
            deserialize_field::<AddressType, ADDRESS_TYPE_SIZE>(&mut bytes_iterator);

        let destination_device_identifier =
            deserialize_field::<AddressType, ADDRESS_TYPE_SIZE>(&mut bytes_iterator);

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
