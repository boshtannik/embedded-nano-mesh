use crate::mesh_lib::node::{
    packet::{Packet, PacketSerializedBytes, Serializer, PACKET_BYTES_SIZE},
    types::ParserBytesBuffer,
};

use super::super::constants::{PACKET_START_BYTE, PACKET_START_BYTES_COUNT};

pub struct PacketBytesParser {
    parsed_packet: Option<Packet>,
    bytes_buffer: ParserBytesBuffer,
}

impl PacketBytesParser {
    pub fn new() -> PacketBytesParser {
        PacketBytesParser {
            parsed_packet: None,
            bytes_buffer: ParserBytesBuffer::new(),
        }
    }

    fn try_parse_packet(&mut self) {
        // No bytes enough yet to be parsed.
        if self.bytes_buffer.len() < (PACKET_START_BYTES_COUNT + PACKET_BYTES_SIZE) {
            return;
        }

        // No start bytes found.
        if !self
            .bytes_buffer
            .iter()
            .take(PACKET_START_BYTES_COUNT)
            .all(|elem| *elem == PACKET_START_BYTE)
        {
            return;
        }

        for _ in 0..PACKET_START_BYTES_COUNT {
            self.bytes_buffer.pop_front().unwrap_or_else(|| 0u8);
        }

        let parsing_buffer: PacketSerializedBytes = self.bytes_buffer.iter().map(|b| *b).collect();

        let got_packet = <Packet as Serializer>::deserialize(parsing_buffer);

        if got_packet.is_checksum_correct() && got_packet.has_correct_source_device_identifier() {
            self.parsed_packet.replace(got_packet);
        }
    }

    pub fn push_byte(&mut self, byte: u8) {
        if self.bytes_buffer.is_full() {
            self.bytes_buffer.pop_front();
        }
        self.bytes_buffer.push_back(byte).unwrap_or_else(|_| {});
        self.try_parse_packet();
    }

    pub fn get_packet(&mut self) -> Option<Packet> {
        self.parsed_packet.take()
    }
}
