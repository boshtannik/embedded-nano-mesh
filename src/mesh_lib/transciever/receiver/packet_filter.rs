use heapless::Vec;

use crate::mesh_lib::{
    millis::{millis, ms},
    transciever::{
        config::PACKET_FILTER_REGISTRATION_SIZE,
        packet::{Packet, PacketError, UniqueId, UniqueIdExtractor},
    },
};

pub struct PacketLifetimeEndedError;
pub struct PacketEntryDuplicationError;

struct PacketRegistrationEntry {
    pub packet_id: UniqueId,
    pub entry_time_end: ms,
}

type PacketRegistrationEntryVec = Vec<PacketRegistrationEntry, PACKET_FILTER_REGISTRATION_SIZE>;

pub struct PacketManager {
    entry_registration_vec: PacketRegistrationEntryVec,
}

impl PacketManager {
    pub fn new() -> PacketManager {
        PacketManager {
            entry_registration_vec: PacketRegistrationEntryVec::new(),
        }
    }

    pub fn decrease_lifetime(&self, packet: Packet) -> Result<Packet, PacketLifetimeEndedError> {
        match packet.deacrease_lifetime() {
            Ok(packet) => Ok(packet),
            Err(PacketError::PacketLifetimeEnded) => Err(PacketLifetimeEndedError),
        }
    }

    pub fn filter_out_duplication(
        &self,
        packet: Packet,
    ) -> Result<Packet, PacketEntryDuplicationError> {
        match self.register_packet_entry(<Packet as UniqueIdExtractor>::get_unique_id(&packet)) {
            Ok(()) => Ok(packet),
            Err(error) => Err(error),
        }
    }

    fn is_entry_present(&self, packet_id: UniqueId) -> bool {
        self.entry_registration_vec
            .iter()
            .any(|entry| entry.packet_id == packet_id)
    }

    fn register_packet_entry(
        &self,
        packet_id: UniqueId,
    ) -> Result<(), PacketEntryDuplicationError> {
        if self.is_entry_present(packet_id) {
            return Err(PacketEntryDuplicationError);
        }
        Ok(())
    }

    pub fn update(&mut self) {
        let current_timme = millis();
        for entry in self.entry_registration_vec.iter_mut() {
            if entry.entry_time_end > current_timme {
                drop(entry);
            }
        }
    }
}
