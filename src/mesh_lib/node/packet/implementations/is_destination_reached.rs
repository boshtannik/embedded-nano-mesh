use super::super::{types::GeneralAddressType, Packet};

impl Packet {
    pub fn is_destination_reached(&self, identifier: GeneralAddressType) -> bool {
        self.destination_device_identifier == identifier.into()
    }
}
