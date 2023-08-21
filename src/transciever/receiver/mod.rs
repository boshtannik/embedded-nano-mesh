mod packet_bytes_parser;

use crate::serial_try_read_byte;
use avr_device::interrupt::Mutex;
use core::cell::{Cell, RefCell};

use self::packet_bytes_parser::PacketBytesParser;

use super::{
    packet::{DataPacker, DeviceIdentifyer, Packet, PacketDataBytes},
    types::{PacketDataQueue, PacketQueue},
};

use arduino_hal::prelude::_embedded_hal_serial_Read;

pub struct Receiver {
    current_device_identifyer: DeviceIdentifyer,
    message_queue: PacketDataQueue,
    transit_packet_queue: RefCell<PacketQueue>,
    packet_bytes_parser: PacketBytesParser,
}

pub enum ReceiverError {
    TransitPacketQueueIsFull,
    MessageQueueIsFull,
    NoPacketToManage,
}

impl Receiver {
    pub fn new(
        current_device_identifyer: DeviceIdentifyer,
        transit_packet_queue: RefCell<PacketQueue>,
    ) -> Receiver {
        Receiver {
            current_device_identifyer,
            message_queue: PacketDataQueue::new(),
            transit_packet_queue,
            packet_bytes_parser: PacketBytesParser::new(),
        }
    }

    pub fn update(&mut self) -> Result<(), ReceiverError> {
        self.receive_byte();
        let maybe_packet = self.packet_bytes_parser.get_packet();
        self.manage_received_packet(maybe_packet)
    }

    pub fn receive(&mut self) -> Option<PacketDataBytes> {
        self.message_queue.pop_front()
    }

    fn manage_received_packet(&mut self, packet: Option<Packet>) -> Result<(), ReceiverError> {
        if let Some(packet) = packet {
            if packet.match_destination_identifyer(&self.current_device_identifyer) {
                match self
                    .message_queue
                    .push_back(<Packet as DataPacker>::unpack(packet))
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(ReceiverError::MessageQueueIsFull),
                }
            } else {
                match self.transit_packet_queue.get_mut().push_back(packet) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(ReceiverError::TransitPacketQueueIsFull),
                }
            }
        } else {
            Err(ReceiverError::NoPacketToManage)
        }
    }

    fn receive_byte(&mut self) {
        let mut mutexed_celled_option_byte: Mutex<Cell<Option<u8>>> = Mutex::new(Cell::new(None));
        serial_try_read_byte!(mutexed_celled_option_byte);

        if let Some(byte) = mutexed_celled_option_byte.get_mut().take() {
            self.packet_bytes_parser.push_byte(byte);
        }
    }
}
