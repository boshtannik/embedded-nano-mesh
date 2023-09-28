mod packet_bytes_parser;
mod packet_filter;

use crate::serial_try_read_byte;
use avr_device::interrupt::Mutex;
use core::cell::Cell;

use self::{
    packet_bytes_parser::PacketBytesParser,
    packet_filter::{Filter, RegistrationError},
};

use super::{
    packet::{DataPacker, Packet},
    PacketMetaData,
};

use arduino_hal::prelude::_embedded_hal_serial_Read;

pub struct Receiver {
    received_packet: Option<PacketMetaData>,
    packet_filter: Filter,
    packet_bytes_parser: PacketBytesParser,
}

pub enum ReceiverError {
    PacketDuplication,
    NoPacketToManage,
    DuplicationFilterOverloaded,
}

impl Receiver {
    pub fn new() -> Receiver {
        Receiver {
            received_packet: None,
            packet_filter: Filter::new(),
            packet_bytes_parser: PacketBytesParser::new(),
        }
    }

    // Checks if packet is duplicated one, and if packet has flag,
    // that tells that that packet needs do be filtered out by the
    // fact of duplication..
    // Raises the error if so, or returns packet Otherwise.
    fn filter_out_duplicated(&mut self, packet: Packet) -> Result<Packet, ReceiverError> {
        match self.packet_filter.filter_out_duplicated(packet) {
            Err(RegistrationError::DuplicationFound) => {
                return Err(ReceiverError::PacketDuplication);
            }
            Err(RegistrationError::RegistrationLimitExceeded) => {
                return Err(ReceiverError::DuplicationFilterOverloaded);
            }
            Ok(packet) => Ok(packet),
        }
    }

    pub fn update(&mut self) -> Result<(), ReceiverError> {
        self._receive_byte();

        self.packet_filter.update();

        let packet = match self.packet_bytes_parser.get_packet() {
            None => return Err(ReceiverError::NoPacketToManage),
            Some(packet) => packet,
        };

        let packet = self.filter_out_duplicated(packet)?;

        self.received_packet
            .replace(<Packet as DataPacker>::unpack(packet));

        Ok(())
    }

    pub fn receive(&mut self) -> Option<PacketMetaData> {
        self.received_packet.take()
    }

    fn _receive_byte(&mut self) {
        let mut mutexed_celled_option_byte: Mutex<Cell<Option<u8>>> = Mutex::new(Cell::new(None));
        serial_try_read_byte!(mutexed_celled_option_byte);

        if let Some(byte) = mutexed_celled_option_byte.get_mut().take() {
            self.packet_bytes_parser.push_byte(byte);
        }
    }
}
