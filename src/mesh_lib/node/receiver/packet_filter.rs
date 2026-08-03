use super::ms;
use heapless::Vec;

use crate::mesh_lib::node::{
    constants::{
        DuplicateFilterDuplicateStrategy, DuplicateFilterOverflowStrategy,
        RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD, RECEIVER_FILTER_DUPLICATE_STRATEGY,
        RECEIVER_FILTER_OVERFLOW_STRATEGY, RECEIVER_FILTER_REGISTRATION_SIZE,
    },
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

impl Clone for PacketIgnorancePeriod {
    fn clone(&self) -> Self {
        PacketIgnorancePeriod {
            packet_unique_id: self.packet_unique_id.clone(),
            registered_at: self.registered_at,
        }
    }
}

type RegistrationEntryVec = Vec<PacketIgnorancePeriod, RECEIVER_FILTER_REGISTRATION_SIZE>;

pub struct Filter {
    // Should be better use hashmaps, but it didn't succeed.
    entry_registration_vec: RegistrationEntryVec,
    /// The currently active ignore period. Starts from the compile-time
    /// configured value and may grow when the `double_period` duplicate
    /// strategy is enabled.
    current_ignore_period: ms,
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            entry_registration_vec: RegistrationEntryVec::new(),
            current_ignore_period: RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD,
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
                Err(RegistrationError::DuplicationFound) => {
                    self._on_duplicate_detected();
                    Err(RegistrationError::DuplicationFound)
                }
                Err(error) => Err(error),
            }
        }
    }

    /// Hook called every time a duplicate of a known packet is detected.
    ///
    /// With the `double_period` strategy this doubles the ignore period,
    /// so a flooding network suppresses echoes for longer and longer.
    fn _on_duplicate_detected(&mut self) {
        match RECEIVER_FILTER_DUPLICATE_STRATEGY {
            DuplicateFilterDuplicateStrategy::Ignore => (),
            DuplicateFilterDuplicateStrategy::DoublePeriod => {
                self.current_ignore_period = self.current_ignore_period.saturating_mul(2);
            }
        }
    }

    pub fn update(&mut self, current_time: ms) {
        let mut index = 0;
        while index < self.entry_registration_vec.len() {
            if current_time.wrapping_sub(self.entry_registration_vec[index].registered_at)
                > self.current_ignore_period
            {
                self.entry_registration_vec.swap_remove(index);
            } else {
                index += 1;
            }
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

        match self.entry_registration_vec.push(new_entry.clone()) {
            Ok(_) => Ok(()),
            Err(_) => match RECEIVER_FILTER_OVERFLOW_STRATEGY {
                DuplicateFilterOverflowStrategy::EvictOldest => {
                    self._evict_oldest_entry(current_time);
                    self.entry_registration_vec
                        .push(new_entry)
                        .map_err(|_| RegistrationError::RegistrationLimitExceeded)
                }
                DuplicateFilterOverflowStrategy::DropNew => {
                    Err(RegistrationError::RegistrationLimitExceeded)
                }
            },
        }
    }

    /// Removes the entry that was registered the earliest.
    fn _evict_oldest_entry(&mut self, current_time: ms) {
        let mut oldest_index = 0;
        let mut oldest_elapsed = current_time.wrapping_sub(self.entry_registration_vec[0].registered_at);
        for (index, entry) in self.entry_registration_vec.iter().enumerate() {
            let elapsed = current_time.wrapping_sub(entry.registered_at);
            if elapsed > oldest_elapsed {
                oldest_elapsed = elapsed;
                oldest_index = index;
            }
        }
        self.entry_registration_vec.swap_remove(oldest_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh_lib::node::packet::{AddressType, LifeTimeType, Packet, PacketDataBytes};

    // Construct a minimal packet with ignore_duplication_flag set.
    // Unique ID is derived from (source_device_identifier, id).
    fn make_packet(source: u8, id: u8) -> Packet {
        Packet::new(
            source as AddressType,
            2 as AddressType,
            id,
            1 as LifeTimeType,
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

    #[test]
    fn all_expired_entries_evicted_in_one_update() {
        let mut filter = Filter::new();
        let t: ms = 1000;
        for id in 0..(RECEIVER_FILTER_REGISTRATION_SIZE as u8) {
            assert!(filter
                .filter_out_duplicated(make_packet(1, id), t)
                .is_ok());
        }
        filter.update(t + RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD + 1);
        // Table is empty now — the oldest packet can be re-registered.
        assert!(filter
            .filter_out_duplicated(make_packet(1, 0), t + RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD + 1)
            .is_ok());
    }

    #[test]
    fn table_full_uses_configured_overflow_strategy() {
        let mut filter = Filter::new();
        let t: ms = 1000;
        for id in 0..(RECEIVER_FILTER_REGISTRATION_SIZE as u8) {
            assert!(filter
                .filter_out_duplicated(make_packet(1, id), t)
                .is_ok());
        }

        let new_packet = make_packet(1, RECEIVER_FILTER_REGISTRATION_SIZE as u8);
        let result = filter.filter_out_duplicated(new_packet, t);

        match RECEIVER_FILTER_OVERFLOW_STRATEGY {
            DuplicateFilterOverflowStrategy::EvictOldest => {
                assert!(result.is_ok(), "new packet must be accepted");
                // Oldest entry (id 0) was evicted, so it can be re-registered.
                assert!(filter
                    .filter_out_duplicated(make_packet(1, 0), t)
                    .is_ok());
            }
            DuplicateFilterOverflowStrategy::DropNew => {
                assert!(result.is_err(), "new packet must be rejected");
            }
        }
    }
}
