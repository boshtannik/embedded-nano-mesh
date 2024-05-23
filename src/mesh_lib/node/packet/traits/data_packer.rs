use super::super::meta_data::PacketMetaData;
use super::super::Packet;

pub struct UnpackSenderAddressError;

pub trait DataPacker {
    fn pack(packet_meta_data: PacketMetaData) -> Packet;
    fn unpack(self) -> Result<PacketMetaData, UnpackSenderAddressError>;
}
