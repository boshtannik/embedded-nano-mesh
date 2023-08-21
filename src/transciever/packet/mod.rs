mod config;
mod traits;
mod types;

use core::ops::Deref;

pub use traits::{DataPacker, PacketSerializer};

pub use config::{CONTENT_SIZE, PACKET_BYTES_SIZE};

use self::types::{AddressType, ChecksumType, FlagsType};

pub use self::types::{PacketDataBytes, PacketSerializedBytes};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceIdentifyer(pub AddressType);
use postcard::{from_bytes, to_vec};
pub use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
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
        to_vec(&self).unwrap()
    }

    fn deserialize(bytes: types::PacketSerializedBytes) -> Self {
        from_bytes(bytes.deref()).unwrap()
    }
}
