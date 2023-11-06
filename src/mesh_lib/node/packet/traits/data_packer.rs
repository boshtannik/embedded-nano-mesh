use super::super::Packet;
use super::super::PacketMetaData;

pub trait DataPacker {
    fn pack(packet_meta_data: PacketMetaData) -> Packet;
    fn unpack(self) -> PacketMetaData;
}
