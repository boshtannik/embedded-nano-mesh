use crate::mesh_lib::{
    millis::{millis, ms},
    transciever::packet::{Packet, PacketError, UniqueId, UniqueIdExtractor},
};

use heapless::FnvIndexMap;

pub struct PacketManager {
    hashmap: FnvIndexMap<UniqueId, ms, 20>,
}

pub enum PacketManagerError {
    Error,
}

impl PacketManager {
    pub fn new() -> PacketManager {
        PacketManager {
            hashmap: FnvIndexMap::new(),
        }
    }

    pub fn decrease_lifetime(&self, packet: Packet) -> Result<Packet, PacketManagerError> {
        match packet.deacrease_lifetime() {
            Ok(packet) => Ok(packet),
            Err(PacketError::PacketLifetimeEnded) => Err(PacketManagerError::Error),
        }
    }

    pub fn filter_out_duplication(&self, packet: Packet) -> Result<Packet, PacketManagerError> {
        let packet_id = <Packet as UniqueIdExtractor>::get_unique_id(&packet);

        match self.register_packet_entry(packet_id) {
            Ok(()) => Ok(packet),
            Err(error) => Err(error),
        }
    }

    fn register_packet_entry(&self, packet_id: UniqueId) -> Result<(), PacketManagerError> {
        // TODO: To be done. Hashmap with Key : Value => UniqueId : registration wipeout millis.
        Err(PacketManagerError::Error)

        // In case if UniqueId is in HashMap -> return Duplication error.
        // In case if packet is newly registered. ->    * Calculate packet ignorance timeout.
        //                                              * Insert values with
        //                                              UniqueId : ignorance timeout
    }

    pub fn update() {
        // TODO: Update HashMap.
        // Wipe out HashMap old time recordings.

        let current_time = millis();
    }
}
