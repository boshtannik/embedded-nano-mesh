use super::super::ExactAddressType;
use super::super::GeneralAddressType;
use super::super::LifeTimeType;
use super::super::PacketDataBytes;
use super::super::PacketState;
use super::traits::StateMutator;
use super::types::IdType;

#[derive(Clone)]
pub struct PacketMetaData {
    pub data: PacketDataBytes,
    pub source_device_identifier: ExactAddressType,
    pub destination_device_identifier: GeneralAddressType,
    pub lifetime: LifeTimeType,
    pub filter_out_duplication: bool,
    pub spec_state: PacketState,
    pub packet_id: IdType,
}

pub struct PacketLifetimeEnded;
pub struct RespondToBroadcastAddressError;

impl PacketMetaData {
    fn swap_source_destination(&mut self) -> Result<(), RespondToBroadcastAddressError> {
        match self.destination_device_identifier {
            // Broadcast target can not do the answers.
            GeneralAddressType::Broadcast => Err(RespondToBroadcastAddressError),

            GeneralAddressType::Exact(destination_device_identifier) => {
                (
                    self.source_device_identifier,
                    self.destination_device_identifier,
                ) = (
                    destination_device_identifier,
                    GeneralAddressType::Exact(self.source_device_identifier),
                );
                Ok(())
            }
        }
    }

    pub fn mutated(mut self) -> Result<Self, RespondToBroadcastAddressError> {
        let old_state = self.spec_state.clone();
        match old_state {
            PacketState::Ping => self.swap_source_destination()?,
            PacketState::SendTransaction | PacketState::InitTransaction => {
                self.swap_source_destination()?;
            }
            PacketState::AcceptTransaction => {
                self.increase_packet_id();
                self.swap_source_destination()?;
            }
            _ => (),
        };
        self.spec_state = old_state.mutated();
        Ok(self)
    }

    pub fn deacrease_lifetime(mut self) -> Result<Self, PacketLifetimeEnded> {
        match self.lifetime.cmp(&1) {
            core::cmp::Ordering::Greater => {
                self.lifetime -= 1;
                Ok(self)
            }
            _ => Err(PacketLifetimeEnded),
        }
    }

    pub fn is_destination_reached(&self, identifier: GeneralAddressType) -> bool {
        self.destination_device_identifier == identifier
    }

    pub fn increase_packet_id(&mut self) {
        (self.packet_id, _) = self.packet_id.overflowing_add(1);
    }
}
