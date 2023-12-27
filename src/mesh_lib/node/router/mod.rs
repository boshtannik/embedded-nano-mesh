pub use super::packet::PacketState;

use super::{
    packet::{
        DataPacker, Packet, PacketMetaData, PacketMetaDataError, StateMutator,
        MULTICAST_RESERVED_IDENTIFIER,
    },
    AddressType, GLOBAL_MUTEXED_CELLED_PACKET_QUEUE,
};

/// Structure which keeps logic of routing of the packets
/// of the network.
///
/// * It handles has the `lifeteime` of the packet.
/// * It hangles packets of different special purposes, like ping-pong, transactions, and handles
/// their further processing.
/// * It transits packets, that were sent to other devices.
pub struct PacketRouter {
    current_device_identifier: AddressType,
}

pub enum OkCase {
    Handled,
    Received(PacketMetaData),
}

pub enum ErrCase {
    TransitQueueIsFull,
    PacketLifetimeEnded,
}

impl PacketRouter {
    pub fn new(current_device_identifier: AddressType) -> Self {
        Self {
            current_device_identifier,
        }
    }

    fn push_to_transit_queue(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        let _ = ::avr_device::interrupt::free(|cs| {
            match GLOBAL_MUTEXED_CELLED_PACKET_QUEUE
                .borrow(cs)
                .borrow_mut()
                .push_back(<Packet as DataPacker>::pack(packet_meta_data))
            {
                Ok(_) => Ok(OkCase::Handled),
                Err(_) => Err(ErrCase::TransitQueueIsFull),
            }
        });
        Ok(OkCase::Handled)
    }

    /// This method is used to handle the packet, as the packet
    /// that have reached it's final destination.
    fn handle_normal(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        return Ok(OkCase::Received(packet_meta_data));
    }

    /// This method is used to handle the packet, that was sent to the all
    /// devices of the network.
    ///
    /// It does few things:
    /// * It saves the copy of the packet to treat it as the packet that was
    /// reached it's destination, and
    /// * Checks if packet can be transferred further, and if so - transfers it further into the
    /// network.
    fn handle_multicast(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        let original_packet_meta_data = packet_meta_data.clone();
        let packet_decreased_lifettime = match packet_meta_data.deacrease_lifetime() {
            Ok(packet_meta_data) => packet_meta_data,
            Err(_) => return Err(ErrCase::PacketLifetimeEnded),
        };
        match self.push_to_transit_queue(packet_decreased_lifettime) {
            Ok(OkCase::Handled) => return Ok(OkCase::Received(original_packet_meta_data)),
            other_err => return other_err,
        }
    }

    /// This method is used to handle the ping packet, that was sent to the
    /// current device.
    ///
    /// It does:
    /// * Gets the packet, that was sent to the current device.
    /// * Decreases lifetime of the packet, and transfers it back to the sender.
    fn handle_ping(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        let original_packet_meta_data = packet_meta_data.clone();
        let packet_decreased_lifettime = match packet_meta_data.deacrease_lifetime() {
            Ok(packet_decreased_lifettime) => packet_decreased_lifettime,
            Err(_) => return Err(ErrCase::PacketLifetimeEnded),
        };
        let mutated_packet_meta_data = packet_decreased_lifettime.mutated();
        self.push_to_transit_queue(mutated_packet_meta_data)?;
        Ok(OkCase::Received(original_packet_meta_data))
    }

    /// This method is used to handle the pong packet, that was sent to the
    /// current device.
    fn handle_pong(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        self.handle_normal(packet_meta_data)
    }

    /// This method is used to handle the send transaction packet, that was sent to the
    /// current device.
    fn handle_send_transaction(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        let packet_decreased_lifettime = match packet_meta_data.deacrease_lifetime() {
            Ok(packet) => packet,
            Err(_) => return Err(ErrCase::PacketLifetimeEnded),
        };
        let mutated_decreased_packet = packet_decreased_lifettime.mutated();
        self.push_to_transit_queue(mutated_decreased_packet)
    }

    /// This method is used to handle the accept transaction packet, that was sent to the
    /// current device.
    fn handle_accept_transaction(
        &self,
        packet_meta_data: PacketMetaData,
    ) -> Result<OkCase, ErrCase> {
        let packet_decreased_lifettime = match packet_meta_data.deacrease_lifetime() {
            Ok(packet) => packet,
            Err(_) => return Err(ErrCase::PacketLifetimeEnded),
        };
        let mutated_decreased_packet = packet_decreased_lifettime.mutated();
        self.push_to_transit_queue(mutated_decreased_packet)
    }

    /// This method is used to handle the accept transaction packet, that was sent to the
    /// current device.
    fn handle_init_transaction(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        let original_packet_meta_data = packet_meta_data.clone();
        let mutated_packet_meta_data = packet_meta_data.mutated();
        self.push_to_transit_queue(mutated_packet_meta_data)?;
        Ok(OkCase::Received(original_packet_meta_data))
    }

    /// This method is used to handle the finish transaction packet, that was sent to the
    /// current device.
    fn handle_finish_transaction(
        &self,
        packet_meta_data: PacketMetaData,
    ) -> Result<OkCase, ErrCase> {
        Ok(OkCase::Received(packet_meta_data))
    }

    /// This method is used to route the packet.
    /// It does:
    /// * Checks if the packet is for the current device, or multicast and handles it.
    /// or
    /// * Checks if the packet can be transferred further, and if so - transfers it further into
    /// the transit queue.
    pub fn route(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        if packet_meta_data.is_destination_identifier_reached(self.current_device_identifier) {
            match packet_meta_data.spec_state {
                PacketState::Normal => self.handle_normal(packet_meta_data),
                PacketState::Ping => self.handle_ping(packet_meta_data),
                PacketState::Pong => self.handle_pong(packet_meta_data),
                PacketState::SendTransaction => self.handle_send_transaction(packet_meta_data),
                PacketState::AcceptTransaction => self.handle_accept_transaction(packet_meta_data),
                PacketState::InitTransaction => self.handle_init_transaction(packet_meta_data),
                PacketState::FinishTransaction => self.handle_finish_transaction(packet_meta_data),
            }
        } else if packet_meta_data.is_destination_identifier_reached(MULTICAST_RESERVED_IDENTIFIER)
        {
            self.handle_multicast(packet_meta_data)
        } else {
            let packet_decreased_lifettime = match packet_meta_data.deacrease_lifetime() {
                Ok(packet_decreased_lifettime) => packet_decreased_lifettime,
                Err(PacketMetaDataError::PacketLifetimeEnded) => {
                    return Err(ErrCase::PacketLifetimeEnded)
                }
            };
            self.push_to_transit_queue(packet_decreased_lifettime)
        }
    }
}
