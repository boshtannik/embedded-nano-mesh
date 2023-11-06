use super::super::{Packet, PacketFlagOps, SpecState};

impl Packet {
    pub fn get_spec_state(&self) -> SpecState {
        if self.is_ping_flag_set() {
            return SpecState::PingPacket;
        }
        if self.is_pong_flag_set() {
            return SpecState::PongPacket;
        }
        if self.is_send_transaction_flag_set() {
            return SpecState::SendTransaction;
        }
        if self.is_accept_transaction_flag_set() {
            return SpecState::AcceptTransaction;
        }
        if self.is_initiate_transaction_flag_set() {
            return SpecState::InitTransaction;
        }
        if self.is_finish_transaction_flag_set() {
            return SpecState::FinishTransaction;
        }
        SpecState::Normal
    }

    pub fn set_spec_state(&mut self, new_state: SpecState) {
        self.set_ping_flag(false);
        self.set_pong_flag(false);
        self.set_send_transaction_flag(false);
        self.set_accept_transaction_flag(false);
        self.set_initiate_transaction_flag(false);
        self.set_finish_transaction_flag(false);
        match new_state {
            SpecState::Normal => (),
            SpecState::PingPacket => {
                self.set_ping_flag(true);
            }
            SpecState::PongPacket => {
                self.set_pong_flag(true);
            }
            SpecState::SendTransaction => {
                self.set_send_transaction_flag(true);
            }
            SpecState::AcceptTransaction => {
                self.set_accept_transaction_flag(true);
            }
            SpecState::InitTransaction => {
                self.set_initiate_transaction_flag(true);
            }
            SpecState::FinishTransaction => {
                self.set_finish_transaction_flag(true);
            }
        }
    }
}
