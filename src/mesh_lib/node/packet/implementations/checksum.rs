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
        let mut result: ChecksumType = 0;

        // Calculate source_device_identifier
        for byte in self.source_device_identifier.to_be_bytes() {
            (result, _) = result.overflowing_add(byte);
        }

        // Calculate destination_device_identifier
        for byte in self.destination_device_identifier.to_be_bytes() {
            (result, _) = result.overflowing_add(byte);
        }

        // Calculate id
        for byte in self.id.to_be_bytes() {
            (result, _) = result.overflowing_add(byte);
        }

        // Calculate lifetime
        for byte in self.lifetime.to_be_bytes() {
            (result, _) = result.overflowing_add(byte);
        }

        // Calculate flags
        for byte in self.flags.to_be_bytes() {
            (result, _) = result.overflowing_add(byte);
        }

        // Calculate data_length
        for byte in self.data_length.to_be_bytes() {
            (result, _) = result.overflowing_add(byte);
        }

        // Calculate data
        for byte in self.data.iter() {
            (result, _) = result.overflowing_add(*byte);
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
