mod config;
mod traits;
mod types;

pub use traits::DataPacker;
pub use traits::PacketBytesSerializer;

pub use config::{CONTENT_SIZE, PACKET_BYTES_SIZE};
pub use types::PacketDataBytes;

use self::types::PacketSerializedBytes;
use self::types::ProtocolVersionType;
use self::types::{AddressType, ChecksumType, FlagsType};
#[derive(Clone, PartialEq, Eq)]
pub struct DeviceIdentifyer(pub AddressType);

/// General data structure to be used to pack messages to
/// be sent over the radio channel.
#[derive(Clone)]
pub struct Packet {
    source_device_identifyer: DeviceIdentifyer,
    destination_device_identifyer: DeviceIdentifyer,
    protocol_version: ProtocolVersionType,
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
            protocol_version: 0,
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
    ///      data_length
    ///      data
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
    ) -> Packet {
        Packet::new(
            source_device_identifyer,
            destination_device_identifyer,
            data,
        )
    }
    fn unpack(packet: Packet) -> PacketDataBytes {
        packet.data
    }
}

impl PacketBytesSerializer for Packet {
    /// Serializing is going in order of keeping all bytes in native endian order.

    fn serialize(self) -> PacketSerializedBytes {
        unimplemented!()
    }

    fn deserialize(bytes: PacketSerializedBytes) -> Packet {
        unimplemented!()
    }
}
