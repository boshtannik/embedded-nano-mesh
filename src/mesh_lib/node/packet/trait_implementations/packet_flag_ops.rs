use super::super::bitpos::{is_flag_set, set_flag};
use super::super::traits::PacketFlagOps;
use super::super::types::{
    ACCEPT_TRANSACTION_FLAG, FINISH_TRANSACTION_FLAG, IGNORE_DUPLICATIONS_FLAG,
    INITIATE_TRANSACTION_FLAG, PING_FLAG, PONG_FLAG, SEND_TRANSACTION_FLAG,
};
use super::super::Packet;

impl PacketFlagOps for Packet {
    // IGNORE_DUPLICATIONS_FLAG
    fn set_ignore_duplication_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, IGNORE_DUPLICATIONS_FLAG, new_state);
    }

    fn is_ignore_duplication_flag_set(&self) -> bool {
        is_flag_set(self.flags, IGNORE_DUPLICATIONS_FLAG)
    }

    // PING_FLAG
    fn set_ping_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, PING_FLAG, new_state);
    }

    fn is_ping_flag_set(&self) -> bool {
        is_flag_set(self.flags, PING_FLAG)
    }

    // PONG_FLAG
    fn set_pong_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, PONG_FLAG, new_state);
    }
    fn is_pong_flag_set(&self) -> bool {
        is_flag_set(self.flags, PONG_FLAG)
    }

    // TRANSACTION_SEND_FLAG
    fn set_send_transaction_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, SEND_TRANSACTION_FLAG, new_state);
    }
    fn is_send_transaction_flag_set(&self) -> bool {
        is_flag_set(self.flags, SEND_TRANSACTION_FLAG)
    }

    // ACCEPT_TRANSACTION_FLAG
    fn set_accept_transaction_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, ACCEPT_TRANSACTION_FLAG, new_state);
    }
    fn is_accept_transaction_flag_set(&self) -> bool {
        is_flag_set(self.flags, ACCEPT_TRANSACTION_FLAG)
    }

    // INITIATE_TRANSACTION_FLAG
    fn set_initiate_transaction_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, INITIATE_TRANSACTION_FLAG, new_state);
    }
    fn is_initiate_transaction_flag_set(&self) -> bool {
        is_flag_set(self.flags, INITIATE_TRANSACTION_FLAG)
    }

    // FINISH_TRANSACTION_FLAG
    fn set_finish_transaction_flag(&mut self, new_state: bool) {
        set_flag(&mut self.flags, FINISH_TRANSACTION_FLAG, new_state);
    }
    fn is_finish_transaction_flag_set(&self) -> bool {
        is_flag_set(self.flags, FINISH_TRANSACTION_FLAG)
    }
}
