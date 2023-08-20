use core::cell::{Cell, RefCell};

use avr_device::interrupt::Mutex;

use crate::serial_try_read_byte;

use super::packet::{DataPacker, DeviceIdentifyer, Packet, PacketDataBytes};

use super::types::{PacketBytesBuffer, PacketDataQueue, PacketQueue};

use arduino_hal::prelude::_embedded_hal_serial_Read;

pub struct Receiver {
    current_device_identifyer: DeviceIdentifyer,
    message_queue: PacketDataQueue,
    received_packet: Option<Packet>,
    transit_packet_queue: RefCell<PacketQueue>,
    received_bytes: PacketBytesBuffer,
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
            received_packet: None,
            transit_packet_queue,
            received_bytes: PacketBytesBuffer::new(),
        }
    }

    pub fn update(&mut self) -> Result<(), ReceiverError> {
        self.receive_byte();
        self.parse_packet();
        self.manage_received_packet()
    }

    pub fn receive(&mut self) -> Option<PacketDataBytes> {
        self.message_queue.pop_front()
    }

    fn parse_packet(&mut self) {
        // Check ring buffer for existance of received packet.
        // Parse packet and check it's crc
    }

    fn manage_received_packet(&mut self) -> Result<(), ReceiverError> {
        if let Some(packet) = self.received_packet.take() {
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
            self.received_bytes.append(byte);
        }
    }
}
