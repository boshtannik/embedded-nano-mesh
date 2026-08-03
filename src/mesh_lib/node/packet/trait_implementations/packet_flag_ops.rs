use super::super::bitpos::{is_flag_set, set_flag};
use super::super::constants::IGNORE_DUPLICATIONS_FLAG;
use super::super::traits::PacketFlagOps;
use super::super::Packet;

impl PacketFlagOps for Packet {
    // IGNORE_DUPLICATIONS_FLAG
    fn set_ignore_duplication_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, IGNORE_DUPLICATIONS_FLAG, new_state);
    }

    fn is_ignore_duplication_flag_set(&self) -> bool {
        is_flag_set(self.flags, IGNORE_DUPLICATIONS_FLAG)
    }
}
