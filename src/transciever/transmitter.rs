use core::cell::RefCell;

use crate::packet::{DeviceIdentifyer, PacketString};

use super::types::{MessageQueue, PacketQueue};

pub struct Transmitter {
    current_device_identifyer: DeviceIdentifyer,
    message_queue: MessageQueue,
    packet_queue: PacketQueue,
    transit_packet_queue: RefCell<PacketQueue>,
}

pub enum Error {
    PacketQueueIsFull,
    MessageQueueIsFull,
}

impl Transmitter {
    pub fn new(
        current_device_identifyer: DeviceIdentifyer,
        transit_packet_queue: RefCell<PacketQueue>,
    ) -> Transmitter {
        Transmitter {
            current_device_identifyer,
            message_queue: MessageQueue::new(),
            packet_queue: PacketQueue::new(),
            transit_packet_queue,
        }
    }

    /*
    pub fn send_message(
        &mut self,
        message: PacketString,
        destination_device_identifyer: DeviceIdentifyer,
    ) -> Result<(), Error> {
        match self.message_queue.push_back(message) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::MessageQueueIsFull),
        }
    }

    pub fn update(&mut self) {
        // Pack messages into packets
        // In case of sending time has come -> Send packets over serial
    }
    */
}
