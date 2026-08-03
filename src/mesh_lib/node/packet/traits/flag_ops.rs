pub trait PacketFlagOps {
    // IGNORE_DUPLICATION_FLAG
    fn set_ignore_duplication_flag(&mut self, new_state: bool);
    fn is_ignore_duplication_flag_set(&self) -> bool;
}
