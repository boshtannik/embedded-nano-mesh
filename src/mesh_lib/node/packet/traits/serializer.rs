use super::super::PacketSerializedBytes;

pub trait Serializer {
    fn serialize(self) -> PacketSerializedBytes;
    fn deserialize(bytes: PacketSerializedBytes) -> Self;
}
