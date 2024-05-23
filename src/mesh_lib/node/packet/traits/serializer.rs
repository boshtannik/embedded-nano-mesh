use super::super::PacketSerializedBytes;

pub trait Serializer {
    fn serialized(self) -> PacketSerializedBytes;
    fn deserialize(bytes: PacketSerializedBytes) -> Self;
}
