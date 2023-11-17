use super::super::AddressType;
use super::super::LifeTimeType;
use super::super::PacketDataBytes;
use super::super::PacketState;
use super::traits::StateMutator;
use super::types::IdType;

#[derive(Clone)]
pub struct PacketMetaData {
    pub data: PacketDataBytes,
    pub source_device_identifier: AddressType,
    pub destination_device_identifier: AddressType,
    pub lifetime: LifeTimeType,
    pub filter_out_duplication: bool,
    pub spec_state: PacketState,
    pub packet_id: IdType,
}

pub enum PacketMetaDataError {
    PacketLifetimeEnded,
}

impl PacketMetaData {
    fn swap_source_destination(&mut self) {
        (
            self.source_device_identifier,
            self.destination_device_identifier,
        ) = (
            self.destination_device_identifier,
            self.source_device_identifier,
        );
    }
    pub fn deacrease_lifetime(mut self) -> Result<Self, PacketMetaDataError> {
        match self.lifetime.cmp(&1) {
            core::cmp::Ordering::Greater => {
                self.lifetime -= 1;
                Ok(self)
            }
            _ => Err(PacketMetaDataError::PacketLifetimeEnded),
        }
    }

    pub fn is_destination_identifier_reached(&self, identifier: AddressType) -> bool {
        self.destination_device_identifier == identifier
    }
}

impl StateMutator for PacketMetaData {
    fn mutated(mut self) -> Self {
        let old_state = self.spec_state.clone();
        match old_state {
            PacketState::Ping
            | PacketState::SendTransaction
            | PacketState::AcceptTransaction
            | PacketState::InitTransaction => self.swap_source_destination(),
            _ => (),
        }
        self.spec_state = old_state.mutated();
        self
    }
}
