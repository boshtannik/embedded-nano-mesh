use crate::serial_write_byte;

use super::config::{PACKET_START_BYTE, PACKET_START_BYTES_COUNT};
use super::packet::{
    DataPacker, DeviceIdentifyer, IdType, LifeTimeType, Packet, PacketDataBytes, Serializer,
};
use super::GLOBAL_MUTEXED_CELLED_QUEUE;

use super::types::PacketQueue;

pub struct Transmitter {
    current_device_identifyer: DeviceIdentifyer,
    packet_queue: PacketQueue,
    id_counter: IdType,
}

pub enum TransmitterError {
    PacketQueueIsFull,
}

impl Transmitter {
    pub fn new(current_device_identifyer: DeviceIdentifyer) -> Transmitter {
        Transmitter {
            current_device_identifyer,
            packet_queue: PacketQueue::new(),
            id_counter: IdType::default(),
        }
    }

    pub fn send(
        &mut self,
        data: PacketDataBytes,
        destination_device_identifyer: DeviceIdentifyer,
        lifetime: LifeTimeType,
    ) -> Result<(), TransmitterError> {
        let (new_val, _) = self.id_counter.overflowing_add(1);
        self.id_counter = new_val;

        let packed_data = <Packet as DataPacker>::pack(
            self.current_device_identifyer.clone(),
            destination_device_identifyer,
            self.id_counter,
            lifetime,
            data,
        );
        match self.packet_queue.push_back(packed_data) {
            Ok(_) => Ok(()),
            Err(_) => Err(TransmitterError::PacketQueueIsFull),
        }
    }

    fn send_start_byte_sequence(&self) {
        for _ in 0..PACKET_START_BYTES_COUNT {
            serial_write_byte!(PACKET_START_BYTE).unwrap_or_else(|_| {});
        }
    }

    pub fn update(&mut self) {
        // Send transit queue
        avr_device::interrupt::free(|cs| {
            while let Some(packet) = GLOBAL_MUTEXED_CELLED_QUEUE
                .borrow(cs)
                .borrow_mut()
                .pop_front()
            {
                self.send_start_byte_sequence();
                for serialized_byte in packet.serialize() {
                    serial_write_byte!(serialized_byte).unwrap_or_else(|_| {});
                }
                return;
            }
        });

        // Send packet queue.
        while let Some(packet) = self.packet_queue.pop_front() {
            self.send_start_byte_sequence();
            for serialized_byte in packet.serialize() {
                serial_write_byte!(serialized_byte).unwrap_or_else(|_| {});
            }
            return;
        }
    }
}
