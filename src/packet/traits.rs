use super::{types::String64Bytes, DeviceAddress, Packet, String64};

pub trait StringPacker {
    fn pack_message(
        source_device_identifyer: DeviceAddress,
        destination_device_identifyer: DeviceAddress,
        message: String64,
    ) -> Packet;
    fn unpack_message(got_packet: Packet) -> String64;
}

pub trait Packer {
    fn pack(
        source_device_identifyer: DeviceAddress,
        destination_device_identifyer: DeviceAddress,
        content: String64Bytes,
    ) -> Packet;

    fn unpack(packet: Packet) -> String64Bytes;
}
