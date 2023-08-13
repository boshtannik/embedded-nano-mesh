use super::{types::PacketStringBytes, DeviceIdentifyer, Packet, PacketString};

pub trait StringPacker {
    fn pack_message(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        message: PacketString,
    ) -> Packet;
    fn unpack_message(got_packet: Packet) -> PacketString;
}

pub trait Packer {
    fn pack(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        content: PacketStringBytes,
    ) -> Packet;

    fn unpack(packet: Packet) -> PacketStringBytes;
}
