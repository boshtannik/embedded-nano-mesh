use crate::{ExactAddressType, GeneralAddressType};

pub use super::packet::PacketState;

use super::packet::{Packet, PacketLifetimeEnded, RespondToBroadcastAddressError};

/// Does the Packet routing of the network.
///
/// * Handles has the `lifeteime` of the packet.
/// * Handles packets of different special purposes, like ping-pong,
///   transactions, and does their further processing.
/// * Catches packets, that were send to this device.
/// * Transits packets, that were sent to other devices.
pub struct Router {
    current_device_identifier: ExactAddressType,
}

pub enum RouteResult {
    ReceivedOnly(Packet),
    TransitOnly(Packet),
    ReceivedAndTransit { received: Packet, transit: Packet },
}

pub enum RouteError {
    PacketLifetimeEnded,
    RespondToBroadcastAddressError,
}

impl From<PacketLifetimeEnded> for RouteError {
    fn from(_: PacketLifetimeEnded) -> Self {
        Self::PacketLifetimeEnded
    }
}

impl From<RespondToBroadcastAddressError> for RouteError {
    fn from(_: RespondToBroadcastAddressError) -> Self {
        Self::RespondToBroadcastAddressError
    }
}

impl Router {
    pub fn new(current_device_identifier: ExactAddressType) -> Self {
        Self {
            current_device_identifier,
        }
    }

    /// This method is used to handle the packet, that was sent to the all
    /// devices of the network.
    ///
    /// It does few things:
    /// * It saves the copy of the packet to treat it as the packet that was
    /// reached it's destination, and
    /// * Checks if packet can be transferred further, and if so - transfers it further into the
    /// network.
    fn handle_broadcast(&self, packet: Packet) -> Result<RouteResult, RouteError> {
        let received = packet.clone();
        let transit: Option<Packet> = match packet.deacrease_lifetime() {
            Ok(packet) => Some(packet),
            Err(PacketLifetimeEnded) => None,
        };
        if let Some(transit) = transit {
            return Ok(RouteResult::ReceivedAndTransit { received, transit });
        }
        return Ok(RouteResult::ReceivedOnly(received));
    }

    fn keep_copy_and_prepare_transit(&self, packet: Packet) -> Result<RouteResult, RouteError> {
        let received = packet.clone();
        let transit = packet.mutated()?;
        Ok(RouteResult::ReceivedAndTransit { received, transit })
    }

    /// This method makes all the packet routing of the netwok.
    /// It does:
    /// * In case, if the packet is addressed to the current device only - handles it.
    /// * In case, if the packet is addressed to the broadcast address:
    ///     1. Catches the packet as received.
    ///     2. Makes copy of received packet - tries to turn it into transit, and sends it back into the
    ///        ether.
    /// * In case, if the packet is addressed to the other device:
    ///     - Reduces lifetime of packet, and in case if packet is still live - sends it
    ///     back into the network.
    pub fn route(&self, packet: Packet) -> Result<RouteResult, RouteError> {
        if packet.is_destination_reached(self.current_device_identifier.into()) {
            return match packet.get_spec_state() {
                PacketState::Normal => Ok(RouteResult::ReceivedOnly(packet)), // No need
                PacketState::Ping => self.keep_copy_and_prepare_transit(packet),
                PacketState::Pong => Ok(RouteResult::ReceivedOnly(packet)),
                PacketState::SendTransaction => Ok(RouteResult::TransitOnly(packet.mutated()?)),
                PacketState::AcceptTransaction => Ok(RouteResult::TransitOnly(packet.mutated()?)),
                PacketState::InitTransaction => self.keep_copy_and_prepare_transit(packet),
                PacketState::FinishTransaction => Ok(RouteResult::ReceivedOnly(packet)),
            };
        }

        if packet.is_destination_reached(GeneralAddressType::Broadcast) {
            return self.handle_broadcast(packet);
        }

        match packet.deacrease_lifetime() {
            Ok(packet) => Ok(RouteResult::TransitOnly(packet)),
            Err(PacketLifetimeEnded) => return Err(RouteError::PacketLifetimeEnded), // Shit happens.
        }
    }
}
