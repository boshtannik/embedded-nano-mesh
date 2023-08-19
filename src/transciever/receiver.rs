use core::cell::RefCell;

use super::packet::{DeviceIdentifyer, PacketDataBytes};

use super::types::{PacketBytesBuffer, PacketDataQueue, PacketQueue};

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

        // In case if packet is ok:
        //      If location is reached - Move out message into message queue.
        //      If location is other - Move packet into transit_packet_queue.
    }

    pub fn receive(&mut self) -> Option<PacketDataBytes> {
        unimplemented!();
    }
}
