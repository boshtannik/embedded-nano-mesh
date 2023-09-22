mod packet_bytes_parser;
mod packet_filter;

use crate::serial_try_read_byte;
use avr_device::interrupt::Mutex;
use core::cell::Cell;

use self::{
    packet_bytes_parser::PacketBytesParser,
    packet_filter::{Filter, PacketLifetimeEndedError, RegistrationError},
};

use super::{
    packet::{DataPacker, DeviceIdentifyer, Packet, PacketDataBytes},
    types::PacketDataQueue,
    BROADCAST_RESERVED_IDENTIFYER, GLOBAL_MUTEXED_CELLED_PACKET_QUEUE,
};

use arduino_hal::prelude::_embedded_hal_serial_Read;

pub struct Receiver {
    current_device_identifyer: DeviceIdentifyer,
    message_queue: PacketDataQueue,
    packet_filter: Filter,
    packet_bytes_parser: PacketBytesParser,
}

pub enum ReceiverError {
    TransitPacketQueueIsFull,
    TransitPacketLifetimeEnded,
    PacketDuplication,
    MessageQueueIsFull,
    NoPacketToManage,
    FilterOverloaded,
}

impl Receiver {
    pub fn new(current_device_identifyer: DeviceIdentifyer) -> Receiver {
        Receiver {
            current_device_identifyer,
            message_queue: PacketDataQueue::new(),
            packet_filter: Filter::new(),
            packet_bytes_parser: PacketBytesParser::new(),
        }
    }

    fn _proceed_duplicated(&mut self, packet: Packet) -> Result<Packet, ReceiverError> {
        match self.packet_filter.filter_out_duplicated(packet) {
            Err(RegistrationError::DuplicationFound) => {
                return Err(ReceiverError::PacketDuplication);
            }
            Err(RegistrationError::RegistrationLimitExceeded) => {
                return Err(ReceiverError::FilterOverloaded);
            }
            Ok(packet) => Ok(packet),
        }
    }

    /// Result->Ok(Some(packet))     - Means, that packet is the transit packet.
    /// Result->Ok(None)             - Means, that the packet has reached it's destination.
    /// Result->Err(ReceiverError)   - Error.
    fn _proceed_destination(&mut self, packet: Packet) -> Result<Option<Packet>, ReceiverError> {
        if !packet.is_destination_identifyer_reached(&self.current_device_identifyer) {
            return Ok(Some(packet));
        }
        match self
            .message_queue
            .push_back(<Packet as DataPacker>::unpack(packet.clone()))
        {
            Ok(_) => return Ok(None),
            Err(_) => return Err(ReceiverError::MessageQueueIsFull),
        }
    }

    fn _proceed_broadcast(&mut self, packet: Packet) -> Result<Packet, ReceiverError> {
        if !packet
            .is_destination_identifyer_reached(&DeviceIdentifyer(BROADCAST_RESERVED_IDENTIFYER))
        {
            return Ok(packet);
        }
        match self
            .message_queue
            .push_back(<Packet as DataPacker>::unpack(packet.clone()))
        {
            Ok(_) => Ok(packet),
            Err(_) => Err(ReceiverError::MessageQueueIsFull),
        }
    }

    fn _proceed_transit(&mut self, packet: Packet) -> Result<(), ReceiverError> {
        let _ = ::avr_device::interrupt::free(|cs| {
            match GLOBAL_MUTEXED_CELLED_PACKET_QUEUE
                .borrow(cs)
                .borrow_mut()
                .push_back(packet)
            {
                Ok(_) => return Ok(()),
                Err(_) => return Err(ReceiverError::TransitPacketQueueIsFull),
            }
        });
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), ReceiverError> {
        self._receive_byte();

        self.packet_filter.update();

        let packet = match self.packet_bytes_parser.get_packet() {
            None => return Err(ReceiverError::NoPacketToManage),
            Some(packet) => packet,
        };

        let packet = self._proceed_duplicated(packet)?;

        let maybe_broadcast_packet = match self._proceed_destination(packet) {
            Ok(None) => return Ok(()),
            Ok(Some(packet)) => packet,
            Err(any_err) => return Err(any_err),
        };

        let transit_packet = self._proceed_broadcast(maybe_broadcast_packet)?;

        let transit_packet = match self.packet_filter.filter_out_lifetime(transit_packet) {
            Err(PacketLifetimeEndedError) => return Err(ReceiverError::TransitPacketLifetimeEnded),
            Ok(packet) => packet,
        };

        self._proceed_transit(transit_packet)
    }

    pub fn receive(&mut self) -> Option<PacketDataBytes> {
        self.message_queue.pop_front()
    }

    fn _receive_byte(&mut self) {
        let mut mutexed_celled_option_byte: Mutex<Cell<Option<u8>>> = Mutex::new(Cell::new(None));
        serial_try_read_byte!(mutexed_celled_option_byte);

        if let Some(byte) = mutexed_celled_option_byte.get_mut().take() {
            self.packet_bytes_parser.push_byte(byte);
        }
    }
}
