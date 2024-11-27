use super::super::Packet;

/// Case, when packet lifetime ended, and the packet can no longer
/// be sent further.
pub struct PacketLifetimeEnded;

impl Packet {
    pub fn deacrease_lifetime(mut self) -> Result<Self, PacketLifetimeEnded> {
        match self.lifetime.cmp(&1) {
            core::cmp::Ordering::Greater => {
                self.lifetime -= 1;
                Ok(self)
            }
            _ => Err(PacketLifetimeEnded),
        }
    }
}
