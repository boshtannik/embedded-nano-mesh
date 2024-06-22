use crate::{ExactAddressType, GeneralAddressType};

pub use super::packet::PacketState;

use super::packet::{PacketLifetimeEnded, PacketMetaData, RespondToBroadcastAddressError};

/// Structure which keeps logic of routing of the packets
/// of the network.
///
/// * It handles has the `lifeteime` of the packet.
/// * It handles packets of different special purposes, like ping-pong,
///   transactions, and does their further processing.
/// * It catches packets, that were send to this device.
/// * It transits packets, that were sent to other devices.
pub struct Router {
    current_device_identifier: ExactAddressType,
}

pub enum RouteResult {
    ReceivedOnly(PacketMetaData),
    TransitOnly(PacketMetaData),
    ReceivedAndTransit {
        received: PacketMetaData,
        transit: PacketMetaData,
    },
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
    fn handle_broadcast(
        &self,
        packet_meta_data: PacketMetaData,
    ) -> Result<RouteResult, RouteError> {
        let received = packet_meta_data.clone();
        let transit: Option<PacketMetaData> = match packet_meta_data.deacrease_lifetime() {
            Ok(packet_meta_data) => Some(packet_meta_data),
            Err(PacketLifetimeEnded) => None,
        };
        if let Some(transit) = transit {
            return Ok(RouteResult::ReceivedAndTransit { received, transit });
        }
        return Ok(RouteResult::ReceivedOnly(received));
    }

    fn keep_copy_and_prepare_transit(
        &self,
        packet_meta_data: PacketMetaData,
    ) -> Result<RouteResult, RouteError> {
        let received = packet_meta_data.clone();
        let transit = packet_meta_data.mutated()?;
        Ok(RouteResult::ReceivedAndTransit { received, transit })
    }

    /// This method makes all the packet routing of the netwok.
    /// It does:
    /// * In case, if the packet is addressed to the current device only - handles it.
    /// * In case, if the packet is addressed to the broadcast:
    ///     - Saves the copy of the packet to treat it as the packet that was
    ///     reached it's destination
    ///     - Checks if packet can be transferred further, and if so - transfers it further into
    ///     the network.
    /// * In case, if the packet is addressed to the other device:
    ///     - Reduces lifetime of packet, and in case if packet is still live - sends it
    ///     back into the network.
    pub fn route(&self, packet_meta_data: PacketMetaData) -> Result<RouteResult, RouteError> {
        if packet_meta_data.is_destination_reached(self.current_device_identifier.into()) {
            return match packet_meta_data.spec_state {
                PacketState::Normal => Ok(RouteResult::ReceivedOnly(packet_meta_data)), // No need
                PacketState::Ping => self.keep_copy_and_prepare_transit(packet_meta_data),
                PacketState::Pong => Ok(RouteResult::ReceivedOnly(packet_meta_data)),
                PacketState::SendTransaction => {
                    Ok(RouteResult::TransitOnly(packet_meta_data.mutated()?))
                }
                PacketState::AcceptTransaction => {
                    Ok(RouteResult::TransitOnly(packet_meta_data.mutated()?))
                }
                PacketState::InitTransaction => {
                    self.keep_copy_and_prepare_transit(packet_meta_data)
                }
                PacketState::FinishTransaction => Ok(RouteResult::ReceivedOnly(packet_meta_data)),
            };
        }

        if packet_meta_data.is_destination_reached(GeneralAddressType::Broadcast) {
            return self.handle_broadcast(packet_meta_data);
        }

        match packet_meta_data.deacrease_lifetime() {
            Ok(packet) => Ok(RouteResult::TransitOnly(packet)),
            Err(PacketLifetimeEnded) => return Err(RouteError::PacketLifetimeEnded), // Shit happens.
        }
    }
}
