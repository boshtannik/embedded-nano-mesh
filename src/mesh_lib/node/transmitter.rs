use platform_serial::PlatformSerial;

use super::constants::{PACKET_START_BYTE, PACKET_START_BYTES_COUNT};
use super::packet::{IdType, Packet, Serializer};

use super::types::PacketQueue;

pub struct Transmitter {
    packet_queue: PacketQueue,
    transit_queue: PacketQueue,
    id_counter: IdType,
}

pub struct PacketQueueIsFull;
pub struct PacketTransitQueueIsFull;

struct QueuePushError;

impl Transmitter {
    pub fn new() -> Transmitter {
        Transmitter {
            packet_queue: PacketQueue::new(),
            transit_queue: PacketQueue::new(),
            id_counter: IdType::default(),
        }
    }

    pub fn send_transit(&mut self, packet: Packet) -> Result<(), PacketTransitQueueIsFull> {
        match self._send(packet, false) {
            Ok(_) => Ok(()),
            Err(QueuePushError) => Err(PacketTransitQueueIsFull),
        }
    }

    pub fn send(&mut self, packet: Packet) -> Result<IdType, PacketQueueIsFull> {
        match self._send(packet, true) {
            Ok(generated_packet_id) => Ok(generated_packet_id),
            Err(QueuePushError) => Err(PacketQueueIsFull),
        }
    }

    fn _send(
        &mut self,
        mut packet: Packet,
        update_id_counter: bool,
    ) -> Result<IdType, QueuePushError> {
        // Packet transition does not require id incrementing.
        // But every other senging method requires it.
        if update_id_counter {
            let (new_val, _) = self.id_counter.overflowing_add(1);
            self.id_counter = new_val;
            packet.set_id(self.id_counter);
        }

        let generated_packet_id = packet.get_id().clone();

        match self.packet_queue.push_back(packet) {
            Ok(_) => Ok(generated_packet_id),
            Err(_) => Err(QueuePushError),
        }
    }

    fn send_start_byte_sequence<SERIAL: PlatformSerial<u8>>(&self) {
        let mut serial = SERIAL::default();
        for _ in 0..PACKET_START_BYTES_COUNT {
            serial.write(PACKET_START_BYTE).unwrap_or_else(|_| {});
        }
    }

    pub fn update<SERIAL: PlatformSerial<u8>>(&mut self) {
        // Send transit queue.
        let mut serial = SERIAL::default();
        while let Some(packet) = self.transit_queue.pop_front() {
            self.send_start_byte_sequence::<SERIAL>();
            for byte in packet.summarized().serialized() {
                serial.write(byte).unwrap_or_else(|_| {})
            }
            return;
        }

        // Send packet queue.
        while let Some(packet) = self.packet_queue.pop_front() {
            self.send_start_byte_sequence::<SERIAL>();
            for byte in packet.summarized().serialized() {
                serial.write(byte).unwrap_or_else(|_| {})
            }
            return;
        }
    }
}
