use super::super::meta_data::PacketMetaData;
use super::super::traits::DataPacker;
use super::super::traits::PacketFlagOps;
use super::super::Packet;

impl DataPacker for Packet {
    fn pack(packet_meta_data: PacketMetaData) -> Self {
        let mut result = Packet::new(
            packet_meta_data.source_device_identifier,
            packet_meta_data.destination_device_identifier,
            packet_meta_data.packet_id,
            packet_meta_data.lifetime,
            packet_meta_data.spec_state,
            packet_meta_data.data,
        );
        result.set_ignore_duplication_flag(packet_meta_data.filter_out_duplication);
        result
    }

    fn unpack(self) -> PacketMetaData {
        PacketMetaData {
            data: self.data.iter().map(|el| *el).collect(), // Can it be simplified?
            source_device_identifier: self.source_device_identifier.clone(),
            destination_device_identifier: self.destination_device_identifier.clone(),
            lifetime: self.lifetime,
            filter_out_duplication: self.is_ignore_duplication_flag_set(),
            spec_state: self.get_spec_state(),
            packet_id: self.id,
        }
    }
}
