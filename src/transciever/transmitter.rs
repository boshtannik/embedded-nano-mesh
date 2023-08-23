use core::cell::RefCell;

use crate::transciever::config::PACKET_START_BYTE;
use crate::{serial_println, serial_write_byte};

use super::config::PACKET_START_BYTES_COUNT;
use super::packet::{DataPacker, DeviceIdentifyer, Packet, PacketDataBytes, PacketSerializer};

use super::types::PacketQueue;

pub struct Transmitter {
    current_device_identifyer: DeviceIdentifyer,
    packet_queue: PacketQueue,
    transit_packet_queue: RefCell<PacketQueue>,
}

pub enum TransmitterError {
    PacketQueueIsFull,
}

impl Transmitter {
    pub fn new(
        current_device_identifyer: DeviceIdentifyer,
        transit_packet_queue: RefCell<PacketQueue>,
    ) -> Transmitter {
        Transmitter {
            current_device_identifyer,
            packet_queue: PacketQueue::new(),
            transit_packet_queue,
        }
    }

    pub fn send(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifyer: DeviceIdentifyer,
    ) -> Result<(), TransmitterError> {
        let packed_data = <Packet as DataPacker>::pack(
            self.current_device_identifyer.clone(),
            destination_device_identifyer,
            data,
        );
        match self.packet_queue.push_back(packed_data) {
            Ok(_) => Ok(()),
            Err(_) => Err(TransmitterError::PacketQueueIsFull),
        }
    }

    fn send_start_byte_sequence(&self) {
        for _ in 0..PACKET_START_BYTES_COUNT {
            serial_write_byte!(PACKET_START_BYTE)
                .unwrap_or_else(|_| serial_println!("Could not write packet byte to serial"));
        }
    }

    pub fn update(&mut self) {
        // Send packet queue.
        while let Some(packet) = self.packet_queue.pop_front() {
            self.send_start_byte_sequence();
            for serialized_byte in packet.serialize() {
                serial_write_byte!(serialized_byte).unwrap_or_else(|_| {
                    serial_println!("Could not write own packet byte into serial")
                });
            }
        }

        // Send transit queue
        while let Some(packet) = self.transit_packet_queue.borrow_mut().pop_front() {
            self.send_start_byte_sequence();
            for serialized_byte in packet.serialize() {
                serial_write_byte!(serialized_byte).unwrap_or_else(|_| {
                    serial_println!("Could not write transit packet byte into serial")
                });
            }
        }
    }
}
