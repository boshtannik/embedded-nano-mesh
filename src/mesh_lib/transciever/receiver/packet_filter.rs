use heapless::Vec;

use crate::{
    mesh_lib::{
        millis::{millis, ms},
        transciever::{
            config::{RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD, RECEIVER_FILTER_REGISTRATION_SIZE},
            packet::{Packet, PacketError, UniqueId, UniqueIdExtractor},
        },
    },
    serial_println,
};

pub struct PacketLifetimeEndedError;

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
    entry_registration_vec: RegistrationEntryVec,
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            entry_registration_vec: RegistrationEntryVec::new(),
        }
    }

    pub fn filter_out_lifetime(&self, packet: Packet) -> Result<Packet, PacketLifetimeEndedError> {
        match packet.deacrease_lifetime() {
            Ok(packet) => Ok(packet),
            Err(PacketError::PacketLifetimeEnded) => Err(PacketLifetimeEndedError),
        }
    }

    pub fn filter_out_duplicated(&mut self, packet: Packet) -> Result<Packet, RegistrationError> {
        match self._register_packet_entry(<Packet as UniqueIdExtractor>::get_unique_id(&packet)) {
            Ok(()) => Ok(packet),
            Err(error) => Err(error),
        }
    }

    pub fn update(&mut self) {
        let current_timme = millis(); // TODO: Hope for better perfomance, might be needed to call
                                      // millis once, and then drop trough the whole library to use same value, void calling
                                      // multiple times.

        let mut index_to_remove: Option<usize> = None;

        for (index, entry) in self.entry_registration_vec.iter().enumerate() {
            if entry.timeout > current_timme {
                index_to_remove.replace(index);
                break;
            }
        }

        if let Some(index_to_remove) = index_to_remove {
            serial_println!("Found item to be cleaned up");
            self.entry_registration_vec.swap_remove(index_to_remove);
        }
    }

    fn _is_entry_present(&self, packet_id: UniqueId) -> bool {
        self.entry_registration_vec
            .iter()
            .any(|entry| entry.packet_id == packet_id)
    }

    fn _register_packet_entry(&mut self, packet_id: UniqueId) -> Result<(), RegistrationError> {
        if self._is_entry_present(packet_id.clone()) {
            return Err(RegistrationError::DuplicationFound);
        }

        let new_entry = PacketIgnorancePeriod {
            packet_id,
            timeout: millis() + RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD,
        };

        match self.entry_registration_vec.push(new_entry) {
            Ok(_) => Ok(()),
            Err(_) => Err(RegistrationError::RegistrationLimitExceeded),
        }
    }
}
