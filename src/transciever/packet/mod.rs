mod config;
mod traits;
mod types;

use core::mem::size_of;

pub use traits::{DataPacker, PacketSerializer};

pub use config::{CONTENT_SIZE, PACKET_BYTES_SIZE};

use crate::serial_debug;

use self::types::{AddressType, ChecksumType, FlagsType};

pub use self::types::{PacketDataBytes, PacketSerializedBytes};

#[derive(Clone, PartialEq, Eq)]
pub struct DeviceIdentifyer(pub AddressType);

#[derive(Clone)]
pub struct Packet {
    source_device_identifyer: DeviceIdentifyer,
    destination_device_identifyer: DeviceIdentifyer,
    flags: FlagsType,
    data_length: usize,
    data: PacketDataBytes,
    checksum: ChecksumType,
}

impl Packet {
    fn new(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        data: PacketDataBytes,
    ) -> Packet {
        let mut new_packet = Packet {
            source_device_identifyer,
            destination_device_identifyer,
            flags: FlagsType::MIN,
            data_length: data.len(),
            data,
            checksum: ChecksumType::MIN,
        };
        new_packet.summarize();
        new_packet
    }

    pub const fn size_of_bytes() -> usize {
        size_of::<DeviceIdentifyer>()
            + size_of::<DeviceIdentifyer>()
            + size_of::<FlagsType>()
            + size_of::<usize>()
            + CONTENT_SIZE
            + size_of::<ChecksumType>()
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
        let (result, _) = result.overflowing_add(self.source_device_identifyer.0);

        // Calculate destination_device_identifyer
        let (mut result, _) = result.overflowing_add(self.destination_device_identifyer.0);

        // Calculate flags
        for byte in self.flags.to_be_bytes() {
            #[allow(irrefutable_let_patterns)]
            if let (new_value, _) = result.overflowing_add(byte) {
                result = new_value;
            }
        }

        // Calculate data_length
        for byte in self.data_length.to_be_bytes() {
            #[allow(irrefutable_let_patterns)]
            if let (new_value, _) = result.overflowing_add(byte) {
                result = new_value;
            }
        }

        // Calculate data
        for byte in self.data.iter() {
            #[allow(irrefutable_let_patterns)]
            if let (new_value, _) = result.overflowing_add(*byte) {
                result = new_value;
            }
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
        data: PacketDataBytes,
    ) -> Self {
        Packet::new(
            source_device_identifyer,
            destination_device_identifyer,
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
        let mut source_device_identifyer: [u8; size_of::<DeviceIdentifyer>()] =
            [0; { size_of::<AddressType>() }];
        for entry in source_device_identifyer.iter_mut() {
            *entry = *bytes_iterator.next().unwrap_or_else(|| {
                serial_debug!("Could not deserialize byte of source_device_identifyer");
                &0u8
            })
        }
        let source_device_identifyer =
            DeviceIdentifyer(AddressType::from_be_bytes(source_device_identifyer));

        // destination_device_identifyer: DeviceIdentifyer,
        let mut destination_device_identifyer: [u8; size_of::<DeviceIdentifyer>()] =
            [0; { size_of::<AddressType>() }];
        for entry in destination_device_identifyer.iter_mut() {
            *entry = *bytes_iterator.next().unwrap_or_else(|| {
                serial_debug!("Could not deserialize byte of destination_device_identifyer");
                &0u8
            })
        }
        let destination_device_identifyer =
            DeviceIdentifyer(AddressType::from_be_bytes(destination_device_identifyer));

        // flags: FlagsType,
        let mut flags: [u8; size_of::<FlagsType>()] = [0; { size_of::<FlagsType>() }];
        for entry in flags.iter_mut() {
            *entry = *bytes_iterator.next().unwrap_or_else(|| {
                serial_debug!("Could not deserialize byte of flags");
                &0u8
            })
        }
        let flags = FlagsType::from_be_bytes(flags);

        // data_length: usize,
        let mut data_length: [u8; size_of::<usize>()] = [0; { size_of::<usize>() }];
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
        let mut checksum: [u8; size_of::<ChecksumType>()] = [0; { size_of::<ChecksumType>() }];
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
            flags,
            data_length,
            data,
            checksum,
        }
    }
}
