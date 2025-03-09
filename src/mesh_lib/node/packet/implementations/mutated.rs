use crate::mesh_lib::node::packet::StateMutator;

use super::super::{
    types::{GeneralAddressType, PacketState},
    Packet,
};

/// Is made to prevent forced response to broadcast address
/// that causes netowrk jamming.
pub struct RespondToBroadcastAddressError;

impl Packet {
    fn swap_source_destination(&mut self) -> Result<(), RespondToBroadcastAddressError> {
        match self.destination_device_identifier.into() {
            // Broadcast target can not do the answers.
            GeneralAddressType::Broadcast => Err(RespondToBroadcastAddressError),
            GeneralAddressType::Exact(destination_device_identifier) => {
                (
                    self.source_device_identifier,
                    self.destination_device_identifier,
                ) = (
                    destination_device_identifier.into(),
                    self.source_device_identifier,
                );
                Ok(())
            }
        }
    }

    pub fn mutated(mut self) -> Result<Self, RespondToBroadcastAddressError> {
        let old_state = self.get_spec_state().clone();

        match old_state {
            PacketState::Ping
            | PacketState::SendTransaction
            | PacketState::InitTransaction
            | PacketState::AcceptTransaction => {
                self.increment_id();
                self.swap_source_destination()?;
            }
            _ => (),
        };
        self.set_spec_state(old_state.mutated());
        Ok(self)
    }
}
