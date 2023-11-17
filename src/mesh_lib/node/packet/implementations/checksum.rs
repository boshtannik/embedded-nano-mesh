use super::super::types::ChecksumType;
use super::super::Packet;

impl Packet {
    /// Checks if the calculated checksum of the packet
    /// matches to the already stored one.
    pub fn is_checksum_correct(&self) -> bool {
        self.calculate_packet_sum() == self.checksum
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
        let result = result.overflowing_add(self.source_device_identifier).0;

        // Calculate destination_device_identifier
        let mut result = result.overflowing_add(self.destination_device_identifier).0;

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
