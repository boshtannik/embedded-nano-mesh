use super::super::constants::{
    ACCEPT_TRANSACTION_FLAG, FINISH_TRANSACTION_FLAG, IGNORE_DUPLICATIONS_FLAG,
    INITIATE_TRANSACTION_FLAG, PING_FLAG, PONG_FLAG, SEND_TRANSACTION_FLAG,
};
use super::super::{Packet, PacketFlagOps, PacketState};

impl Packet {
    pub fn get_spec_state(&self) -> PacketState {
        if self.is_ping_flag_set() {
            return PacketState::Ping;
        }
        if self.is_pong_flag_set() {
            return PacketState::Pong;
        }
        if self.is_send_transaction_flag_set() {
            return PacketState::SendTransaction;
        }
        if self.is_accept_transaction_flag_set() {
            return PacketState::AcceptTransaction;
        }
        if self.is_initiate_transaction_flag_set() {
            return PacketState::InitTransaction;
        }
        if self.is_finish_transaction_flag_set() {
            return PacketState::FinishTransaction;
        }
        PacketState::Normal
    }

    pub fn set_spec_state(&mut self, new_state: PacketState) {
        self.flags = (self.flags & IGNORE_DUPLICATIONS_FLAG) | state_flag(new_state);
    }
}

fn state_flag(new_state: PacketState) -> u8 {
    match new_state {
        PacketState::Normal => 0,
        PacketState::Ping => PING_FLAG,
        PacketState::Pong => PONG_FLAG,
        PacketState::SendTransaction => SEND_TRANSACTION_FLAG,
        PacketState::AcceptTransaction => ACCEPT_TRANSACTION_FLAG,
        PacketState::InitTransaction => INITIATE_TRANSACTION_FLAG,
        PacketState::FinishTransaction => FINISH_TRANSACTION_FLAG,
    }
}
