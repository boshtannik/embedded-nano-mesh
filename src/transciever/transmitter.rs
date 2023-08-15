use core::cell::RefCell;

use crate::{serial_println, serial_write_byte};

use super::packet::{
    DeviceIdentifyer, Packet, PacketBytesSerializer, PacketString, PacketStringSerializer,
};

use super::types::PacketQueue;

pub struct Transmitter {
    current_device_identifyer: DeviceIdentifyer,
    packet_queue: PacketQueue,
    transit_packet_queue: RefCell<PacketQueue>,
}

pub enum Error {
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

    pub fn send_message(
        &mut self,
        message: PacketString,
        destination_device_identifyer: DeviceIdentifyer,
    ) -> Result<(), Error> {
        let packed_message = <Packet as PacketStringSerializer>::pack(
            self.current_device_identifyer.clone(),
            destination_device_identifyer,
            message,
        );
        match self.packet_queue.push_back(packed_message) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::PacketQueueIsFull),
        }
    }

    /// Needs to be called in the loop.
    /// This method sends packets, once they are in the queues.
    /// It sends packets, that were created inside of this node, and
    /// sends packets, which are collected from other nodes, and their
    /// destination node is other than this one.
    pub fn update(&mut self) {
        // Send packet queue.
        while let Some(packet) = self.packet_queue.pop_front() {
            for byte in packet.serialize() {
                serial_write_byte!(byte)
                    .unwrap_or_else(|_| serial_println!("Could not write packet byte into serial"));
            }
        }

        // Send transit queue
        while let Some(packet) = self.transit_packet_queue.borrow_mut().pop_front() {
            for byte in packet.serialize() {
                serial_write_byte!(byte).unwrap_or_else(|_| {
                    serial_println!("Could not write transit packet byte into serial")
                });
            }
        }
    }
}
