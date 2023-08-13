mod config;
mod traits;
mod types;

pub use types::String64;

use crate::serial_println;

use self::{
    traits::{Packer, StringPacker},
    types::{AddressType, ChecksumType, FlagsType, String64Bytes},
};
#[derive(Clone)]
pub struct DeviceAddress(pub AddressType);

#[derive(Clone)]
pub struct Packet {
    source_device_identifyer: DeviceAddress,
    destination_device_identifyer: DeviceAddress,
    protocol_version: u8,
    flags: FlagsType,
    content_length: usize,
    content: String64Bytes,
    checksum: ChecksumType,
}

impl Packet {
    pub fn new(
        source_device_identifyer: DeviceAddress,
        destination_device_identifyer: DeviceAddress,
        content: String64Bytes,
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

impl Packer for Packet {
    fn pack(
        source_device_identifyer: DeviceAddress,
        destination_device_identifyer: DeviceAddress,
        content: String64Bytes,
    ) -> Packet {
        Packet::new(
            source_device_identifyer,
            destination_device_identifyer,
            content,
        )
    }
    fn unpack(packet: Packet) -> String64Bytes {
        packet.content
    }
}

impl StringPacker for Packet {
    fn pack_message(
        source_device_identifyer: DeviceAddress,
        destination_device_identifyer: DeviceAddress,
        message: String64,
    ) -> Packet {
        <Packet as Packer>::pack(
            source_device_identifyer,
            destination_device_identifyer,
            message.into_bytes(),
        )
    }

    /// As long as hepless::String type consist of 1 byte characters:
    /// So the new string will be created, and filled byte by bytre characters.
    fn unpack_message(got_packet: Packet) -> String64 {
        let mut result = String64::new();
        for byte in got_packet.content.iter() {
            result.push(*byte as char).unwrap_or_else(|_| {
                serial_println!("Error in StringPacker trait in unpack_message in pushing byte to result string").unwrap();
            })
        }
        result
    }
}
