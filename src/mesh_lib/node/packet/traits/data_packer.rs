use super::super::meta_data::PacketMetaData;
use super::super::Packet;

pub trait DataPacker {
    fn pack(packet_meta_data: PacketMetaData) -> Packet;
    fn unpack(self) -> PacketMetaData;
}
