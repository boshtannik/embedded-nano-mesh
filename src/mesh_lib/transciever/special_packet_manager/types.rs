pub enum SpecPacketState {
    Normal,
    PingPacket,
    PongPacket,
    SendTransactionPacket,
    AcceptTransactionPacket,
    InitTransactionPacket,
    FinishTransactionPacket,
}
