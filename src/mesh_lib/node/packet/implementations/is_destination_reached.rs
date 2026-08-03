use super::super::{types::GeneralAddressType, Packet};

impl Packet {
    /// Checks whether the packet is addressed to the given destination.
    pub fn is_destination_reached(&self, identifier: GeneralAddressType) -> bool {
        self.destination_device_identifier == identifier.into()
    }
}
