use platform_serial_arduino_nano::{serial_write_byte, ArduinoNanoSerial};

use super::constants::{PACKET_START_BYTE, PACKET_START_BYTES_COUNT};
use super::packet::{DataPacker, IdType, Packet, PacketFlagOps, Serializer};
use super::{PacketMetaData, GLOBAL_MUTEXED_CELLED_PACKET_QUEUE};

use embedded_hal::serial::Write;

use super::types::PacketQueue;

pub struct Transmitter {
    packet_queue: PacketQueue,
    id_counter: IdType,
}

pub enum TransmitterError {
    PacketQueueIsFull,
}

impl Transmitter {
    pub fn new() -> Transmitter {
        Transmitter {
            packet_queue: PacketQueue::new(),
            id_counter: IdType::default(),
        }
    }

    pub fn send(&mut self, mut packet_meta_data: PacketMetaData) -> Result<(), TransmitterError> {
        let (new_val, _) = self.id_counter.overflowing_add(1);
        self.id_counter = new_val;

        packet_meta_data.packet_id = self.id_counter;

        let filter_out_duplication = packet_meta_data.filter_out_duplication;
        let mut packed_data = <Packet as DataPacker>::pack(packet_meta_data);

        packed_data.set_ignore_duplication_flag(filter_out_duplication);

        match self.packet_queue.push_back(packed_data) {
            Ok(_) => Ok(()),
            Err(_) => Err(TransmitterError::PacketQueueIsFull),
        }
    }

    fn send_start_byte_sequence(&self) {
        for _ in 0..PACKET_START_BYTES_COUNT {
            serial_write_byte!(ArduinoNanoSerial::default(), PACKET_START_BYTE)
                .unwrap_or_else(|_| {});
        }
    }

    pub fn update(&mut self) {
        // Send transit queue
        avr_device::interrupt::free(|cs| {
            while let Some(packet) = GLOBAL_MUTEXED_CELLED_PACKET_QUEUE
                .borrow(cs)
                .borrow_mut()
                .pop_front()
            {
                self.send_start_byte_sequence();
                for serialized_byte in packet.summarized().serialize() {
                    serial_write_byte!(ArduinoNanoSerial::default(), serialized_byte)
                        .unwrap_or_else(|_| {});
                }
                return;
            }
        });

        // Send packet queue.
        while let Some(packet) = self.packet_queue.pop_front() {
            self.send_start_byte_sequence();
            for serialized_byte in packet.summarized().serialize() {
                serial_write_byte!(ArduinoNanoSerial::default(), serialized_byte)
                    .unwrap_or_else(|_| {});
            }
            return;
        }
    }
}
