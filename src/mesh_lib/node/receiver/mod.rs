mod packet_bytes_parser;
mod packet_filter;

use super::constants::RECEIVER_READER_BUFFER_SIZE;

use super::ms;

use self::{
    packet_bytes_parser::PacketBytesParser,
    packet_filter::{Filter, RegistrationError},
};

use super::Packet;

pub struct Receiver {
    packet_filter: Filter,
    packet_bytes_parser: PacketBytesParser,
}

enum ReceiverError {
    PacketDuplication,
    DuplicationFilterOverloaded,
}

impl Receiver {
    pub fn new() -> Receiver {
        Receiver {
            packet_filter: Filter::new(),
            packet_bytes_parser: PacketBytesParser::new(),
        }
    }

    // Checks if packet is duplicated one, and if packet has flag,
    // that tells that that packet needs do be filtered out by the
    // fact of duplication..
    // Raises the error if so, or returns packet Otherwise.
    fn filter_out_duplicated(
        &mut self,
        packet: Packet,
        current_time: ms,
    ) -> Result<Packet, ReceiverError> {
        match self
            .packet_filter
            .filter_out_duplicated(packet, current_time)
        {
            Err(RegistrationError::DuplicationFound) => {
                return Err(ReceiverError::PacketDuplication);
            }
            Err(RegistrationError::RegistrationLimitExceeded) => {
                return Err(ReceiverError::DuplicationFilterOverloaded);
            }
            Ok(packet) => Ok(packet),
        }
    }

    /// Does the following:
    /// - reads byte from serial
    /// - checks if the packet can be built from with new received byte.
    pub fn update<I>(&mut self, current_time: ms, interface_driver: &mut I)
    where
        I: embedded_io::ReadReady + embedded_io::Read + embedded_io::Write,
    {
        self._receive_byte(interface_driver);
        self.packet_filter.update(current_time);
    }

    /// Checks, if parser has packet being parsed, and then
    /// cheks if packet is not duplicated.
    /// Returns packet if all checks were passed, or None otherwise.
    pub fn receive(&mut self, current_time: ms) -> Option<Packet> {
        let packet = match self.packet_bytes_parser.get_packet() {
            None => return None,
            Some(packet) => packet,
        };

        let packet = match self.filter_out_duplicated(packet, current_time) {
            Err(_) => return None,
            Ok(packet) => packet,
        };

        Some(packet)
    }

    fn _receive_byte<I>(&mut self, interface_driver: &mut I)
    where
        I: embedded_io::ReadReady + embedded_io::Read + embedded_io::Write,
    {
        // No read guard
        match interface_driver.read_ready() {
            Ok(false) | Err(_) => return,
            Ok(true) => (),
        }

        let mut buf = [0u8; RECEIVER_READER_BUFFER_SIZE];

        // Empty read guard
        if let Ok(red_count) = interface_driver.read(&mut buf) {
            if red_count < RECEIVER_READER_BUFFER_SIZE {
                return;
            }
        }

        for b in buf {
            self.packet_bytes_parser.push_byte(b);
        }
    }
}
