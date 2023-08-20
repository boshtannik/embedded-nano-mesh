use core::cell::{Cell, RefCell};

use avr_device::interrupt::Mutex;

use crate::{serial_println, serial_try_read_byte, serial_write_byte};

use super::packet::{DeviceIdentifyer, PacketDataBytes};

use super::types::{PacketBytesBuffer, PacketDataQueue, PacketQueue};

use arduino_hal::prelude::_embedded_hal_serial_Read;

pub struct Receiver {
    current_device_identifyer: DeviceIdentifyer,
    message_queue: PacketDataQueue,
    packet_queue: PacketQueue,
    transit_packet_queue: RefCell<PacketQueue>,
    received_bytes: PacketBytesBuffer,
}

pub enum Error {
    PacketQueueIsFull,
    MessageQueueIsFull,
}

impl Receiver {
    pub fn new(
        current_device_identifyer: DeviceIdentifyer,
        transit_packet_queue: RefCell<PacketQueue>,
    ) -> Receiver {
        Receiver {
            current_device_identifyer,
            message_queue: PacketDataQueue::new(),
            packet_queue: PacketQueue::new(),
            transit_packet_queue,
            received_bytes: PacketBytesBuffer::new(),
        }
    }

    pub fn update(&mut self) {
        // Check received packets.
        // In case if packet is corrupt -> drop it.

        let mut mutexed_celled_option: Mutex<Cell<Option<u8>>> = Mutex::new(Cell::new(None));
        serial_try_read_byte!(mutexed_celled_option);

        if let Some(byte) = mutexed_celled_option.get_mut().take() {
            serial_write_byte!(byte).unwrap_or_else(|_| serial_println!("Could not echo byte"));
        }

        // In case if packet is ok:
        //      If location is reached - Move out message into message queue.
        //      If location is other - Move packet into transit_packet_queue.
    }

    pub fn receive(&mut self) -> Option<PacketDataBytes> {
        None
        // unimplemented!();
    }
}
