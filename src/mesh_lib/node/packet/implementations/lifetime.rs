use super::super::Packet;

/// Case, when packet lifetime ended, and the packet can no longer
/// be sent further.
pub struct PacketLifetimeEnded;

impl Packet {
    /// Decreases the packet lifetime by one hop.
    ///
    /// Returns `Err(PacketLifetimeEnded)` when the lifetime cannot be
    /// decreased further, meaning the packet shall be destroyed.
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
