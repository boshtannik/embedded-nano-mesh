use super::super::ExactAddressType;
use super::super::GeneralAddressType;
use super::super::LifeTimeType;
use super::super::PacketDataBytes;
use super::super::PacketState;
use super::traits::StateMutator;
use super::types::IdType;

#[derive(Clone)]
/// This is the raw structure, to store
/// packet meta data. It is yet to be serialized into
/// `Packet` instance and then into bytes in order to be
/// sent over the network.
///
/// Also it is created from bytes and then from `Packet`
/// instance when received from the network and deserialized.
pub struct PacketMetaData {
    /// Bytes of packet data.
    pub data: PacketDataBytes,

    /// Address of device, that sends the packet.
    pub source_device_identifier: ExactAddressType,

    /// Address of device, that the packet is addressed to.
    pub destination_device_identifier: GeneralAddressType,

    /// Amount of devices, that this packet will pass through.
    pub lifetime: LifeTimeType,

    /// Tells if the protocol on the other devices will be ignoring
    /// echoes of this message. It is strongly recommended to use
    /// in order to make lower load onto the network.
    pub filter_out_duplication: bool,

    /// Is used to tells if the packet is acquired by
    /// ping, transaction or normal sending
    /// operation.
    pub spec_state: PacketState,

    /// Is used by the network to tell one packet
    /// from another in order to filter out
    /// duplicated packets.
    pub packet_id: IdType,
}

/// Case, when packet lifetime ended, and the packet can no longer
/// be sent further.
pub struct PacketLifetimeEnded;

/// Is made to prevent forced response to broadcast address
/// that causes netowrk jamming.
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
