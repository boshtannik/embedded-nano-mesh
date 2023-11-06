use super::super::PacketState;
use super::super::StateMutator;

impl StateMutator for PacketState {
    fn mutated(self) -> Self {
        match self {
            Self::Ping => Self::Pong,
            Self::SendTransaction => Self::AcceptTransaction,
            Self::AcceptTransaction => Self::InitTransaction,
            Self::InitTransaction => Self::FinishTransaction,
            _ => self,
        }
    }
}
