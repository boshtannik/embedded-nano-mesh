use crate::{
    serial_println,
    transciever::{
        packet::{Packet, PacketSerializedBytes, PacketSerializer, PACKET_BYTES_SIZE},
        types::PacketBytesBuffer,
    },
};

use super::super::config::{PACKET_START_BYTE, PACKET_START_BYTES_COUNT};

pub struct PacketBytesParser {
    parsed_packet: Option<Packet>,
    bytes_buffer: PacketBytesBuffer,
}

impl PacketBytesParser {
    pub fn new() -> PacketBytesParser {
        PacketBytesParser {
            parsed_packet: None,
            bytes_buffer: PacketBytesBuffer::new(),
        }
    }

    fn try_parse_packet(&mut self) {
        if self.bytes_buffer.len() < (PACKET_START_BYTES_COUNT + PACKET_BYTES_SIZE) {
            return;
        }

        if !self
            .bytes_buffer
            .iter()
            .take(PACKET_START_BYTES_COUNT)
            .all(|elem| *elem == PACKET_START_BYTE)
        {
            return;
        }

        let mut parsing_buffer = PacketSerializedBytes::new();

        for i in 0..=PACKET_START_BYTES_COUNT + PACKET_BYTES_SIZE {
            if i <= PACKET_START_BYTES_COUNT {
                continue;
            }

            let popped_element = self.bytes_buffer.pop_front().unwrap_or_else(|| {
                serial_println!("Error. Could not pop from bytes_buffer");
                0u8
            });

            parsing_buffer.push(popped_element).unwrap_or_else(|_| {
                serial_println!("Error. Could not push byte into the parsing_buffer")
            });
        }

        let got_packet = <Packet as PacketSerializer>::deserialize(parsing_buffer);

        if got_packet.is_checksum_correct() {
            self.parsed_packet.replace(got_packet);
        }
    }

    pub fn push_byte(&mut self, byte: u8) {
        if self.bytes_buffer.is_full() {
            self.bytes_buffer.pop_front();
        }
        self.bytes_buffer
            .push_back(byte)
            .unwrap_or_else(|_| serial_println!("ERROR: Could not push received byte into buffer"));
        self.try_parse_packet();
    }

    pub fn get_packet(&mut self) -> Option<Packet> {
        self.parsed_packet.take()
    }
}
