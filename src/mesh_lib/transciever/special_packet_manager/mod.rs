mod types;

pub use types::SpecPacketState;

use super::packet::Packet;

pub struct SpecialPacketManager;

impl SpecialPacketManager {
    fn transform(packet: Packet) -> Packet {
        packet
    }
}
