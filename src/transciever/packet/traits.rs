use super::{
    types::{PacketDataBytes, PacketSerializedBytes},
    DeviceIdentifyer, Packet,
};

pub trait DataPacker {
    fn pack(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        data: PacketDataBytes,
    ) -> Packet;

    fn unpack(packet: Packet) -> PacketDataBytes;
}

pub trait PacketBytesSerializer {
    fn serialize(self: Self) -> PacketSerializedBytes;
    fn deserialize(bytes: PacketSerializedBytes) -> Packet;
}
