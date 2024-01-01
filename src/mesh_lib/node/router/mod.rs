pub use super::packet::PacketState;

use super::{
    packet::{PacketMetaData, PacketMetaDataError, StateMutator, MULTICAST_RESERVED_IDENTIFIER},
    AddressType,
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

pub enum RouteResult {
    Received(PacketMetaData),
    Transit(PacketMetaData),
    ReceivedAndTransit {
        received: PacketMetaData,
        transit: PacketMetaData,
    },
}

pub struct PacketLifetimeEnded;

impl PacketRouter {
    pub fn new(current_device_identifier: AddressType) -> Self {
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
    fn handle_multicast(
        &self,
        packet_meta_data: PacketMetaData,
    ) -> Result<RouteResult, PacketLifetimeEnded> {
        let received = packet_meta_data.clone();
        let transit: Option<PacketMetaData> = match packet_meta_data.deacrease_lifetime() {
            Ok(packet_meta_data) => Some(packet_meta_data),
            Err(PacketMetaDataError::PacketLifetimeEnded) => None,
        };
        if let Some(transit) = transit {
            return Ok(RouteResult::ReceivedAndTransit { received, transit });
        }
        return Ok(RouteResult::Received(received));
    }

    fn keep_copy_and_prepare_transit(
        &self,
        packet_meta_data: PacketMetaData,
    ) -> Result<RouteResult, PacketLifetimeEnded> {
        let received = packet_meta_data.clone();
        let transit = packet_meta_data.mutated();
        Ok(RouteResult::ReceivedAndTransit { received, transit })
    }

    /// This method is routes the packet.
    /// It does:
    /// * Checks if the packet is addressed current device, or is multicast - and handles it.
    /// or
    /// * Checks if the packet can be transferred further, and if so - transfers it further into
    /// the transit queue.
    pub fn route(
        &self,
        packet_meta_data: PacketMetaData,
    ) -> Result<RouteResult, PacketLifetimeEnded> {
        if packet_meta_data.is_destination_identifier_reached(self.current_device_identifier) {
            match packet_meta_data.spec_state {
                PacketState::Normal => Ok(RouteResult::Received(packet_meta_data)), // No need
                PacketState::Ping => self.keep_copy_and_prepare_transit(packet_meta_data),
                PacketState::Pong => Ok(RouteResult::Received(packet_meta_data)),
                PacketState::SendTransaction => {
                    Ok(RouteResult::Transit(packet_meta_data.mutated()))
                }
                PacketState::AcceptTransaction => {
                    Ok(RouteResult::Transit(packet_meta_data.mutated()))
                }
                PacketState::InitTransaction => {
                    self.keep_copy_and_prepare_transit(packet_meta_data)
                }
                PacketState::FinishTransaction => Ok(RouteResult::Received(packet_meta_data)),
            }
        } else if packet_meta_data.is_destination_identifier_reached(MULTICAST_RESERVED_IDENTIFIER)
        {
            self.handle_multicast(packet_meta_data)
        } else {
            let packet_decreased_lifettime = match packet_meta_data.deacrease_lifetime() {
                Ok(packet_decreased_lifettime) => packet_decreased_lifettime,
                Err(PacketMetaDataError::PacketLifetimeEnded) => return Err(PacketLifetimeEnded), // Shit happens.
            };
            Ok(RouteResult::Transit(packet_decreased_lifettime))
        }
    }
}
