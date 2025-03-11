use super::super::{Packet, PacketFlagOps, PacketState};

impl Packet {
    pub fn get_spec_state(&self) -> PacketState {
        PacketState::Normal
    }

    pub fn set_spec_state(&mut self, _new_state: PacketState) {
        let is_ignore_duplication_flag_set = self.is_ignore_duplication_flag_set();
        self.flags = 0;
        self.set_ignore_duplication_flag(is_ignore_duplication_flag_set);
    }
}
