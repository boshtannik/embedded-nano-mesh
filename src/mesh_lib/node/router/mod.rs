use crate::{ExactAddressType, GeneralAddressType};

use super::packet::{Packet, PacketLifetimeEnded};

/// Does the Packet routing of the network.
///
/// * Handles has the `lifeteime` of the packet.
/// * Catches packets, that were send to this device.
/// * Transits packets, that were sent to other devices.
pub struct Router {
    current_device_identifier: ExactAddressType,
}

#[derive(Debug)]
pub enum RouteResult {
    ReceivedOnly(Packet),
    TransitOnly(Packet),
    ReceivedAndTransit { received: Packet, transit: Packet },
}

#[derive(Debug)]
pub enum RouteError {
    PacketLifetimeEnded,
}

impl From<PacketLifetimeEnded> for RouteError {
    fn from(_: PacketLifetimeEnded) -> Self {
        Self::PacketLifetimeEnded
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

    pub fn route(&self, packet: Packet) -> Result<RouteResult, RouteError> {
        if packet.is_destination_reached(self.current_device_identifier.into()) {
            return Ok(RouteResult::ReceivedOnly(packet));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh_lib::node::packet::{Packet, PacketDataBytes};

    fn make_packet(
        source: u8,
        destination: u8,
        lifetime: u8,
        ignore_duplication: bool,
    ) -> Packet {
        Packet::new(
            source,
            destination,
            1,
            lifetime,
            ignore_duplication,
            PacketDataBytes::new(),
        )
    }

    #[test]
    fn packet_for_self_is_received_only() {
        let router = Router::new(ExactAddressType::new(2).unwrap());
        let result = router.route(make_packet(1, 2, 3, true)).unwrap();

        match result {
            RouteResult::ReceivedOnly(_) => (),
            other => panic!("expected ReceivedOnly, got {:?}", other),
        }
    }

    #[test]
    fn packet_for_other_is_transit_only() {
        let router = Router::new(ExactAddressType::new(2).unwrap());
        let result = router.route(make_packet(1, 3, 3, true)).unwrap();

        match result {
            RouteResult::TransitOnly(packet) => assert_eq!(packet.get_lifetime(), 2),
            other => panic!("expected TransitOnly, got {:?}", other),
        }
    }

    #[test]
    fn lifetime_ended_packet_is_dropped() {
        let router = Router::new(ExactAddressType::new(2).unwrap());
        let result = router.route(make_packet(1, 3, 1, true));

        assert!(matches!(result, Err(RouteError::PacketLifetimeEnded)));
    }

    #[test]
    fn broadcast_is_received_and_transited() {
        let router = Router::new(ExactAddressType::new(2).unwrap());
        let result = router.route(make_packet(1, 0, 3, true)).unwrap();

        match result {
            RouteResult::ReceivedAndTransit { received, transit } => {
                assert_eq!(received.source_device_identifier, 1);
                assert_eq!(transit.get_lifetime(), 2);
            }
            other => panic!("expected ReceivedAndTransit, got {:?}", other),
        }
    }

    #[test]
    fn broadcast_with_lifetime_one_is_received_only() {
        let router = Router::new(ExactAddressType::new(2).unwrap());
        let result = router.route(make_packet(1, 0, 1, true)).unwrap();

        match result {
            RouteResult::ReceivedOnly(_) => (),
            other => panic!("expected ReceivedOnly, got {:?}", other),
        }
    }
}
