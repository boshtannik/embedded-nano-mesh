use super::{
    types::{PacketSerializedBytes, PacketStringBytes},
    DeviceIdentifyer, Packet, PacketString,
};

pub trait PacketStringSerializer {
    fn pack(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        message: PacketString,
    ) -> Packet;
    fn unpack(got_packet: Packet) -> PacketString;
}

pub trait PacketByteSerializer {
    fn pack(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        content: PacketStringBytes,
    ) -> Packet;

    fn unpack(packet: Packet) -> PacketStringBytes;
}

pub trait PacketBytesSerializer {
    fn serialize(self: Self) -> PacketSerializedBytes;
    fn deserialize(bytes: PacketSerializedBytes) -> Packet;
}
