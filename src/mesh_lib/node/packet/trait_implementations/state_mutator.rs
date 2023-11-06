use super::super::SpecState;
use super::super::StateMutator;

impl StateMutator for SpecState {
    fn mutated(self) -> Self {
        match self {
            Self::PingPacket => Self::PongPacket,
            Self::SendTransaction => Self::AcceptTransaction,
            Self::AcceptTransaction => Self::InitTransaction,
            Self::InitTransaction => Self::FinishTransaction,
            _ => self,
        }
    }
}
