mod config;
mod traits;
mod types;

pub use traits::PacketByteSerializer;
pub use traits::PacketBytesSerializer;
pub use traits::PacketStringSerializer;

pub use types::PacketString;

use crate::serial_println;

use self::types::PacketSerializedBytes;
use self::types::ProtocolVersionType;
use self::types::{AddressType, ChecksumType, FlagsType, PacketStringBytes};
#[derive(Clone)]
pub struct DeviceIdentifyer(pub AddressType);

/// General data structure to be used to pack messages to
/// be sent over the radio channel.
#[derive(Clone)]
pub struct Packet {
    source_device_identifyer: DeviceIdentifyer,
    destination_device_identifyer: DeviceIdentifyer,
    protocol_version: ProtocolVersionType,
    flags: FlagsType,
    content_length: usize,
    content: PacketStringBytes,
    checksum: ChecksumType,
}

impl Packet {
    pub fn new(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        content: PacketStringBytes,
    ) -> Packet {
        let mut new_packet = Packet {
            source_device_identifyer,
            destination_device_identifyer,
            protocol_version: 0,
            flags: FlagsType::MIN,
            content_length: content.len(),
            content,
            checksum: ChecksumType::MIN,
        };
        new_packet.summarize();
        new_packet
    }

    /// Checks if the calculated checksum of the packet
    /// matches to the already stored one.
    fn is_checksum_correct(&self) -> bool {
        self.calculate_packet_sum() == self.checksum
    }

    /// Calculates and returns checksum of whole packet.
    ///
    /// Checksum consist of next fields:
    ///      source_device_identifyer
    ///      destination_device_identifyer
    ///      protocol_version
    ///      flags
    ///      content_length
    ///      content
    fn calculate_packet_sum(&self) -> ChecksumType {
        let result: ChecksumType = 0;

        // Calculate source_device_identifyer
        let (result, _) = result.overflowing_add(self.source_device_identifyer.0);

        // Calculate destination_device_identifyer
        let (result, _) = result.overflowing_add(self.destination_device_identifyer.0);

        // Calculate protocol_version
        let (mut result, _) = result.overflowing_add(self.protocol_version);

        // Calculate flags
        for byte in self.flags.to_be_bytes() {
            #[allow(irrefutable_let_patterns)]
            if let (new_value, _) = result.overflowing_add(byte) {
                result = new_value;
            }
        }

        // Calculate content_length
        for byte in self.content_length.to_be_bytes() {
            #[allow(irrefutable_let_patterns)]
            if let (new_value, _) = result.overflowing_add(byte) {
                result = new_value;
            }
        }

        // Calculate content
        for byte in self.content.iter() {
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

impl PacketByteSerializer for Packet {
    fn pack(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        content: PacketStringBytes,
    ) -> Packet {
        Packet::new(
            source_device_identifyer,
            destination_device_identifyer,
            content,
        )
    }
    fn unpack(packet: Packet) -> PacketStringBytes {
        packet.content
    }
}

impl PacketStringSerializer for Packet {
    fn pack(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        message: PacketString,
    ) -> Packet {
        <Packet as PacketByteSerializer>::pack(
            source_device_identifyer,
            destination_device_identifyer,
            message.into_bytes(),
        )
    }

    /// As long as hepless::String type consist of 1 byte characters:
    /// So the new string will be created, and filled byte by bytre characters.
    fn unpack(got_packet: Packet) -> PacketString {
        let mut result = PacketString::new();
        for byte in got_packet.content.iter() {
            result.push(*byte as char).unwrap_or_else(|_| {})
        }
        result
    }
}

impl PacketBytesSerializer for Packet {
    /// Serializing is going in order of keeping all bytes in native endian order.

    fn serialize(self) -> PacketSerializedBytes {
        let mut result = PacketSerializedBytes::new();
        // source_device_identifyer,
        // destination_device_identifyer,
        // protocol_version: 0,
        // flags: FlagsType::MIN,
        // content_length: content.len(),
        // content,
        // checksum: ChecksumType::MIN,

        for byte in self.source_device_identifyer.0.to_ne_bytes() {
            result.push(byte).unwrap_or_else(|_| {
                serial_println!(
                    "Unable to add byte to result during source_device_identifyer serialization"
                )
            });
        }

        for byte in self.destination_device_identifyer.0.to_ne_bytes() {
            result.push(byte).unwrap_or_else(|_| {
                serial_println!("Unable to add byte to result during destination_device_identifyer serialization")
            });
        }

        for byte in self.protocol_version.to_ne_bytes() {
            result.push(byte).unwrap_or_else(|_| {
                serial_println!(
                    "Unable to add byte to result during protocol_version serialization"
                )
            });
        }

        for byte in self.flags.to_ne_bytes() {
            result.push(byte).unwrap_or_else(|_| {
                serial_println!("Unable to add byte to result during flags serialization")
            });
        }

        for byte in self.content_length.to_ne_bytes() {
            result.push(byte).unwrap_or_else(|_| {
                serial_println!("Unable to add byte to result during content_length serialization")
            });
        }

        for byte in self.content {
            result.push(byte).unwrap_or_else(|_| {
                serial_println!("Unable to add byte to result during content serialization")
            });
        }

        for byte in self.checksum.to_ne_bytes() {
            result.push(byte).unwrap_or_else(|_| {
                serial_println!("Unable to add byte to result during checksum serialization")
            });
        }

        result
    }

    fn deserialize(bytes: PacketSerializedBytes) -> Packet {
        unimplemented!();
    }
}
