use super::{
    types::{LifeTimeType, PacketDataBytes, PacketSerializedBytes},
    DeviceIdentifyer, Packet,
};

pub trait DataPacker {
    fn pack(
        source_device_identifyer: DeviceIdentifyer,
        destination_device_identifyer: DeviceIdentifyer,
        lifetime: LifeTimeType,
        data: PacketDataBytes,
    ) -> Packet;

    fn unpack(self: Self) -> PacketDataBytes;
}

pub trait PacketSerializer {
    fn serialize(self) -> PacketSerializedBytes;
    fn deserialize(bytes: PacketSerializedBytes) -> Self;
}
