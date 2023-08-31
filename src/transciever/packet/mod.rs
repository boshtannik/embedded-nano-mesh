mod config;
mod traits;
mod types;

use core::mem::size_of;

pub use traits::{DataPacker, PacketSerializer};

pub use config::{CONTENT_SIZE, PACKET_BYTES_COUNT};

use crate::serial_debug;

use self::types::{AddressType, ChecksumType, FlagsType};

pub use self::types::{LifeTimeType, PacketDataBytes, PacketSerializedBytes};

#[derive(Clone, PartialEq, Eq)]
pub struct DeviceIdentifyer(pub AddressType);

#[derive(Clone)]
pub struct Packet {
    source_device_identifyer: DeviceIdentifyer,
    destination_device_identifyer: DeviceIdentifyer,
    lifetime: LifeTimeType,
    flags: FlagsType,
    data_length: usize,
    data: PacketDataBytes,
    checksum: ChecksumType,
}

const DEVICE_IDENTIFYER_TYPE_SIZE: usize = size_of::<DeviceIdentifyer>();
const LIFETIME_TYPE_SIZE: usize = size_of::<LifeTimeType>();
const FLAGS_TYPE_SIZE: usize = size_of::<FlagsType>();
const DATA_LENGTH_TYPE_SIZE: usize = size_of::<usize>();
// data_type_size: CONTENT_SIZE
const CHECKSUM_TYPE_SIZE: usize = size_of::<ChecksumType>();

pub enum PacketError {
    PacketLifetimeEnded,
}

impl Packet {
    fn new(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        lifetime: LifeTimeType,
        data: PacketDataBytes,
    ) -> Packet {
        let mut new_packet = Packet {
            source_device_identifyer,
            destination_device_identifyer,
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
            + LIFETIME_TYPE_SIZE
            + FLAGS_TYPE_SIZE
            + DATA_LENGTH_TYPE_SIZE
            + CONTENT_SIZE
            + CHECKSUM_TYPE_SIZE
    }

    pub fn deacrease_lifetime(&mut self) -> Result<(), PacketError> {
        match self.lifetime.cmp(&1) {
            core::cmp::Ordering::Greater => {
                self.lifetime -= 1;
                self.summarize();
                Ok(())
            }
            _ => Err(PacketError::PacketLifetimeEnded),
        }
    }

    /// Checks if the calculated checksum of the packet
    /// matches to the already stored one.
    pub fn is_checksum_correct(&self) -> bool {
        self.calculate_packet_sum() == self.checksum
    }

    pub fn match_destination_identifyer(&self, identifyer: &DeviceIdentifyer) -> bool {
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
        lifetime: LifeTimeType,
        data: PacketDataBytes,
    ) -> Self {
        Packet::new(
            source_device_identifyer,
            destination_device_identifyer,
            lifetime,
            data,
        )
    }

    fn unpack(self: Self) -> PacketDataBytes {
        self.data
    }
}

impl PacketSerializer for Packet {
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

        // source_device_identifyer: DeviceIdentifyer,
        let mut source_device_identifyer: [u8; DEVICE_IDENTIFYER_TYPE_SIZE] =
            [0; DEVICE_IDENTIFYER_TYPE_SIZE];
        for entry in source_device_identifyer.iter_mut() {
            *entry = *bytes_iterator.next().unwrap_or_else(|| {
                serial_debug!("Could not deserialize byte of source_device_identifyer");
                &0u8
            })
        }
        let source_device_identifyer =
            DeviceIdentifyer(AddressType::from_be_bytes(source_device_identifyer));

        // destination_device_identifyer: DeviceIdentifyer,
        let mut destination_device_identifyer: [u8; DEVICE_IDENTIFYER_TYPE_SIZE] =
            [0; DEVICE_IDENTIFYER_TYPE_SIZE];
        for entry in destination_device_identifyer.iter_mut() {
            *entry = *bytes_iterator.next().unwrap_or_else(|| {
                serial_debug!("Could not deserialize byte of destination_device_identifyer");
                &0u8
            })
        }
        let destination_device_identifyer =
            DeviceIdentifyer(AddressType::from_be_bytes(destination_device_identifyer));

        // lifetime: LifeTimeType,
        let mut lifetime: [u8; LIFETIME_TYPE_SIZE] = [0; LIFETIME_TYPE_SIZE];
        for entry in lifetime.iter_mut() {
            *entry = *bytes_iterator.next().unwrap_or_else(|| {
                serial_debug!("Could not deserialize byte of lifetime");
                &0u8
            })
        }
        let lifetime = FlagsType::from_be_bytes(lifetime);

        // flags: FlagsType,
        let mut flags: [u8; FLAGS_TYPE_SIZE] = [0; FLAGS_TYPE_SIZE];
        for entry in flags.iter_mut() {
            *entry = *bytes_iterator.next().unwrap_or_else(|| {
                serial_debug!("Could not deserialize byte of flags");
                &0u8
            })
        }
        let flags = FlagsType::from_be_bytes(flags);

        // data_length: usize,
        let mut data_length: [u8; DATA_LENGTH_TYPE_SIZE] = [0; DATA_LENGTH_TYPE_SIZE];
        for entry in data_length.iter_mut() {
            *entry = *bytes_iterator.next().unwrap_or_else(|| {
                serial_debug!("Could not deserialize byte of data_length");
                &0u8
            })
        }
        let data_length = usize::from_be_bytes(data_length);

        // data: PacketDataBytes,
        let mut data: PacketDataBytes = PacketDataBytes::new();
        for _ in 0..CONTENT_SIZE {
            data.push(*bytes_iterator.next().unwrap_or_else(|| {
                serial_debug!("Could not take byte for deserialization of data");
                &0u8
            }))
            .unwrap_or_else(|_| {
                serial_debug!("Could not push byte of serialized data");
            });
        }

        // checksum: ChecksumType,
        let mut checksum: [u8; CHECKSUM_TYPE_SIZE] = [0; CHECKSUM_TYPE_SIZE];
        for entry in checksum.iter_mut() {
            *entry = *bytes_iterator.next().unwrap_or_else(|| {
                serial_debug!("Could not deserialize byte of checksum");
                &0u8
            })
        }
        let checksum = ChecksumType::from_be_bytes(checksum);

        Packet {
            source_device_identifyer,
            destination_device_identifyer,
            lifetime,
            flags,
            data_length,
            data,
            checksum,
        }
    }
}
