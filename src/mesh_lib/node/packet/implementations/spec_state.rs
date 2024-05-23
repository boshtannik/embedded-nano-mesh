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
        self.flags = 0;
        match new_state {
            PacketState::Normal => (),
            PacketState::Ping => self.set_ping_flag(true),
            PacketState::Pong => self.set_pong_flag(true),
            PacketState::SendTransaction => self.set_send_transaction_flag(true),
            PacketState::AcceptTransaction => self.set_accept_transaction_flag(true),
            PacketState::InitTransaction => self.set_initiate_transaction_flag(true),
            PacketState::FinishTransaction => self.set_finish_transaction_flag(true),
        }
    }
}
