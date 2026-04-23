use super::ms;
use heapless::Vec;

use crate::mesh_lib::node::{
    constants::{RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD, RECEIVER_FILTER_REGISTRATION_SIZE},
    packet::{Packet, PacketFlagOps, PacketUniqueId, UniqueIdExtractor},
};

pub enum RegistrationError {
    RegistrationLimitExceeded,
    DuplicationFound,
}

struct PacketIgnorancePeriod {
    pub packet_unique_id: PacketUniqueId,
    pub registered_at: ms,
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
            if current_time.wrapping_sub(entry.registered_at)
                > RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD
            {
                index_to_remove.replace(index);
                break;
            }
        }

        if let Some(index_to_remove) = index_to_remove {
            self.entry_registration_vec.swap_remove(index_to_remove);
        }
    }

    fn _is_entry_present(&self, packet_id: PacketUniqueId) -> bool {
        self.entry_registration_vec
            .iter()
            .any(|entry| entry.packet_unique_id == packet_id)
    }

    fn _register_packet_entry(
        &mut self,
        packet_unique_identifier: PacketUniqueId,
        current_time: ms,
    ) -> Result<(), RegistrationError> {
        if self._is_entry_present(packet_unique_identifier.clone()) {
            return Err(RegistrationError::DuplicationFound);
        }

        let new_entry = PacketIgnorancePeriod {
            packet_unique_id: packet_unique_identifier,
            registered_at: current_time,
        };

        match self.entry_registration_vec.push(new_entry) {
            Ok(_) => Ok(()),
            Err(_) => Err(RegistrationError::RegistrationLimitExceeded),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh_lib::node::packet::{Packet, PacketDataBytes, PacketState};

    // Construct a minimal packet with ignore_duplication_flag set.
    // Unique ID is derived from (source_device_identifier, id).
    fn make_packet(source: u8, id: u8) -> Packet {
        Packet::new(
            source,
            2,
            id,
            1,
            PacketState::Normal,
            true,
            PacketDataBytes::new(),
        )
    }

    #[test]
    fn duplicate_detected_within_ignore_period() {
        let mut filter = Filter::new();
        let t: ms = 1000;
        let packet = make_packet(1, 0);
        assert!(filter.filter_out_duplicated(packet.clone(), t).is_ok());
        filter.update(t + RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD - 1);
        assert!(filter
            .filter_out_duplicated(packet, t + RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD - 1)
            .is_err());
    }

    #[test]
    fn entry_expires_after_ignore_period() {
        let mut filter = Filter::new();
        let t: ms = 1000;
        let packet = make_packet(1, 0);
        assert!(filter.filter_out_duplicated(packet.clone(), t).is_ok());
        filter.update(t + RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD + 1);
        // Entry gone — same packet can be re-registered
        assert!(filter
            .filter_out_duplicated(packet, t + RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD + 1)
            .is_ok());
    }

    #[test]
    fn duplicate_detected_within_period_across_u32_wraparound() {
        let mut filter = Filter::new();
        // 500ms before overflow
        let near_max: ms = u32::MAX - 500;
        let packet = make_packet(1, 0);
        assert!(filter
            .filter_out_duplicated(packet.clone(), near_max)
            .is_ok());

        let t_100ms = near_max.wrapping_add(100);
        filter.update(t_100ms);
        assert!(
            filter.filter_out_duplicated(packet, t_100ms).is_err(),
            "entry evicted after 100ms; old code overflows deadline to 499 then fires immediately"
        );
    }

    #[test]
    fn entry_expires_after_period_across_u32_wraparound() {
        let mut filter = Filter::new();
        // 500ms before overflow
        let near_max: ms = u32::MAX - 500;
        let packet = make_packet(1, 0);
        assert!(filter
            .filter_out_duplicated(packet.clone(), near_max)
            .is_ok());

        let t_after = near_max.wrapping_add(RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD + 1);
        filter.update(t_after);
        assert!(
            filter.filter_out_duplicated(packet, t_after).is_ok(),
            "entry not evicted after 1001ms across wraparound"
        );
    }
}
