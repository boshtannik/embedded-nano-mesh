pub trait PacketFlagOps {
    // IGNORE_DUPLICATION_FLAG
    fn set_ignore_duplication_flag(&mut self, new_state: bool);
    fn is_ignore_duplication_flag_set(&self) -> bool;

    // PING_FLAG
    fn set_ping_flag(&mut self, new_state: bool);
    fn is_ping_flag_set(&self) -> bool;

    // PONG_FLAG
    fn set_pong_flag(&mut self, new_state: bool);
    fn is_pong_flag_set(&self) -> bool;

    // SEND_TRANSACTION_FLAG
    fn set_send_transaction_flag(&mut self, new_state: bool);
    fn is_send_transaction_flag_set(&self) -> bool;

    // ACCEPT_TRANSACTION_FLAG
    fn set_accept_transaction_flag(&mut self, new_state: bool);
    fn is_accept_transaction_flag_set(&self) -> bool;

    // INITIATE_TRANSACTION_FLAG
    fn set_initiate_transaction_flag(&mut self, new_state: bool);
    fn is_initiate_transaction_flag_set(&self) -> bool;

    // FINISH_TRANSACTION_FLAG
    fn set_finish_transaction_flag(&mut self, new_state: bool);
    fn is_finish_transaction_flag_set(&self) -> bool;
}
