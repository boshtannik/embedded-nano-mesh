mod packet_bytes_parser;
mod packet_filter;

use crate::serial_try_read_byte;
use avr_device::interrupt::Mutex;
use core::cell::Cell;

use self::{packet_bytes_parser::PacketBytesParser, packet_filter::PacketManager};

use super::{
    packet::{DataPacker, DeviceIdentifyer, Packet, PacketDataBytes},
    types::PacketDataQueue,
    GLOBAL_MUTEXED_CELLED_QUEUE,
};

use arduino_hal::prelude::_embedded_hal_serial_Read;

pub struct Receiver {
    current_device_identifyer: DeviceIdentifyer,
    message_queue: PacketDataQueue,
    packet_manager: PacketManager,
    packet_bytes_parser: PacketBytesParser,
}

pub enum ReceiverError {
    TransitPacketQueueIsFull,
    TransitPacketLifetimeEnded,
    PacketDuplication,
    MessageQueueIsFull,
    NoPacketToManage,
}

impl Receiver {
    pub fn new(current_device_identifyer: DeviceIdentifyer) -> Receiver {
        Receiver {
            current_device_identifyer,
            message_queue: PacketDataQueue::new(),
            packet_manager: PacketManager::new(),
            packet_bytes_parser: PacketBytesParser::new(),
        }
    }

    pub fn update(&mut self) -> Result<(), ReceiverError> {
        self.receive_byte();

        let packet = match self.packet_bytes_parser.get_packet() {
            None => return Err(ReceiverError::NoPacketToManage),
            Some(packet) => packet,
        };

        let packet = match self.packet_manager.filter_out_duplication(packet) {
            Err(_) => return Err(ReceiverError::PacketDuplication),
            Ok(packet) => packet,
        };

        if packet.is_destination_identifyer_reached(&self.current_device_identifyer) {
            match self
                .message_queue
                .push_back(<Packet as DataPacker>::unpack(packet))
            {
                Ok(_) => return Ok(()),
                Err(_) => return Err(ReceiverError::MessageQueueIsFull),
            };
        }

        let packet = match self.packet_manager.decrease_lifetime(packet) {
            Err(_) => return Err(ReceiverError::TransitPacketLifetimeEnded),
            Ok(packet) => packet,
        };

        let _ = ::avr_device::interrupt::free(|cs| {
            match GLOBAL_MUTEXED_CELLED_QUEUE
                .borrow(cs)
                .borrow_mut()
                .push_back(packet)
            {
                Ok(_) => Ok(()),
                Err(_) => Err(ReceiverError::TransitPacketQueueIsFull),
            }
        });

        Ok(())
    }

    pub fn receive(&mut self) -> Option<PacketDataBytes> {
        self.message_queue.pop_front()
    }

    fn receive_byte(&mut self) {
        let mut mutexed_celled_option_byte: Mutex<Cell<Option<u8>>> = Mutex::new(Cell::new(None));
        serial_try_read_byte!(mutexed_celled_option_byte);

        if let Some(byte) = mutexed_celled_option_byte.get_mut().take() {
            self.packet_bytes_parser.push_byte(byte);
        }
    }
}
