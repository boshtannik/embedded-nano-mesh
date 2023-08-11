mod config;
use heapless::Vec;

use self::config::{AddressType, ChecksumType, FlagsType};
pub struct DeviceAddress(pub AddressType);

pub use config::String64;

pub struct Packet {
    source_device_identifyer: DeviceAddress,
    destination_device_identifyer: DeviceAddress,
    protocol_version: u8,
    flags: FlagsType,
    content_length: usize,
    content: Vec<u8, { config::CONTENT_SIZE }>,
    checksum: ChecksumType,
}

impl Packet {
    pub fn new(
        source_device_identifyer: DeviceAddress,
        destination_device_identifyer: DeviceAddress,
        content: Vec<u8, { config::CONTENT_SIZE }>,
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
