use heapless::Vec;
use platform_millis::ms;

use crate::mesh_lib::node::{
    constants::{RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD, RECEIVER_FILTER_REGISTRATION_SIZE},
    packet::{Packet, PacketFlagOps, UniqueId, UniqueIdExtractor},
};

pub enum RegistrationError {
    RegistrationLimitExceeded,
    DuplicationFound,
}

struct PacketIgnorancePeriod {
    pub packet_id: UniqueId,
    pub timeout: ms,
}

type RegistrationEntryVec = Vec<PacketIgnorancePeriod, RECEIVER_FILTER_REGISTRATION_SIZE>;

pub struct Filter {
    // Should be better use hashmaps, but it didn't succeed.
    entry_registration_vec: RegistrationEntryVec,
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            entry_registration_vec: RegistrationEntryVec::new(),
        }
    }

    pub fn filter_out_duplicated(
        &mut self,
        packet: Packet,
        current_time: ms,
    ) -> Result<Packet, RegistrationError> {
        if !packet.is_ignore_duplication_flag_set() {
            Ok(packet)
        } else {
            match self._register_packet_entry(
                <Packet as UniqueIdExtractor>::get_unique_id(&packet),
                current_time,
            ) {
                Ok(()) => Ok(packet),
                Err(error) => Err(error),
            }
        }
    }

    pub fn update(&mut self, current_time: ms) {
        let mut index_to_remove: Option<usize> = None;

        for (index, entry) in self.entry_registration_vec.iter().enumerate() {
            if entry.timeout > current_time {
                index_to_remove.replace(index);
                break;
            }
        }

        if let Some(index_to_remove) = index_to_remove {
            self.entry_registration_vec.swap_remove(index_to_remove);
        }
    }

    fn _is_entry_present(&self, packet_id: UniqueId) -> bool {
        self.entry_registration_vec
            .iter()
            .any(|entry| entry.packet_id == packet_id)
    }

    fn _register_packet_entry(
        &mut self,
        packet_id: UniqueId,
        current_time: ms,
    ) -> Result<(), RegistrationError> {
        if self._is_entry_present(packet_id.clone()) {
            return Err(RegistrationError::DuplicationFound);
        }

        let new_entry = PacketIgnorancePeriod {
            packet_id,
            timeout: current_time + RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD,
        };

        match self.entry_registration_vec.push(new_entry) {
            Ok(_) => Ok(()),
            Err(_) => Err(RegistrationError::RegistrationLimitExceeded),
        }
    }
}
