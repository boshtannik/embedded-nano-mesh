use crate::mesh_lib::transciever::packet::StateMutator;

#[derive(PartialEq, Eq, Clone)]
pub enum SpecPacketState {
    Normal,
    PingPacket,
    PongPacket,
    SendTransaction,
    AcceptTransaction,
    InitTransaction,
    FinishTransaction,
}

impl StateMutator for SpecPacketState {
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
