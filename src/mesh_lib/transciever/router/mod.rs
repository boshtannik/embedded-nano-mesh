mod types;

pub use types::SpecState;

use super::{
    packet::{DataPacker, Packet, StateMutator, BROADCAST_RESERVED_IDENTIFYER},
    DeviceIdentifyer, PacketMetaData, GLOBAL_MUTEXED_CELLED_PACKET_QUEUE,
};

pub struct PacketRouter {
    current_device_identifyer: DeviceIdentifyer,
}

pub enum OkCase {
    Handled,
    Received(PacketMetaData),
}

pub enum ErrCase {
    TransitQueueIsFull,
    PacketLifetimeEnded,
}

impl PacketRouter {
    pub fn new(current_device_identifyer: DeviceIdentifyer) -> Self {
        Self {
            current_device_identifyer,
        }
    }

    fn push_to_transit_queue(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        let _ = ::avr_device::interrupt::free(|cs| {
            match GLOBAL_MUTEXED_CELLED_PACKET_QUEUE
                .borrow(cs)
                .borrow_mut()
                .push_back(<Packet as DataPacker>::pack(packet_meta_data))
            {
                Ok(_) => Ok(OkCase::Handled),
                Err(_) => Err(ErrCase::TransitQueueIsFull),
            }
        });
        Ok(OkCase::Handled)
    }

    fn handle_normal(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        if packet_meta_data.destination_device_identifyer == self.current_device_identifyer {
            return Ok(OkCase::Received(packet_meta_data));
        } else {
            let packet_decreased_lifettime = match packet_meta_data.deacrease_lifetime() {
                Ok(packet_meta_data) => packet_meta_data,
                Err(_) => return Err(ErrCase::PacketLifetimeEnded),
            };

            self.push_to_transit_queue(packet_decreased_lifettime)
        }
    }

    fn handle_broadcast(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        let original_packet_meta_data = packet_meta_data.clone();
        let packet_decreased_lifettime = match packet_meta_data.deacrease_lifetime() {
            Ok(packet_meta_data) => packet_meta_data,
            Err(_) => return Err(ErrCase::PacketLifetimeEnded),
        };
        match self.push_to_transit_queue(packet_decreased_lifettime) {
            Ok(OkCase::Handled) => return Ok(OkCase::Received(original_packet_meta_data)),
            other_err => return other_err,
        }
    }

    fn handle_ping(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        let original_packet_meta_data = packet_meta_data.clone();
        let packet_decreased_lifettime = match packet_meta_data.deacrease_lifetime() {
            Ok(packet_decreased_lifettime) => packet_decreased_lifettime,
            Err(_) => return Err(ErrCase::PacketLifetimeEnded),
        };
        let mutated_packet_meta_data = packet_decreased_lifettime.mutated();
        self.push_to_transit_queue(mutated_packet_meta_data)?;
        Ok(OkCase::Received(original_packet_meta_data))
    }

    fn handle_pong(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        self.handle_normal(packet_meta_data)
    }

    /*
    fn handle_send_transaction(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        // I have received SendTransaction
        // decrease lifetime
        // mutate packet_meta_data
        // push_to_transit_queue
        unimplemented!()
    }

    fn handle_accept_transaction(
        &self,
        packet_meta_data: PacketMetaData,
    ) -> Result<OkCase, ErrCase> {
        // I have received AcceptTransaction
        // deacrease_lifetime
        // mutate packet_meta_data
        // push_to_transit_queue
        unimplemented!()
    }

    fn handle_init_transaction(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        // I have received InitTransaction
        // save original packet
        // Do not decrease lifetime
        // mutate packet
        // push_to_transit_queue
        // return original_packet_meta_data
        unimplemented!()
    }

    fn handle_finish_transaction(
        &self,
        packet_meta_data: PacketMetaData,
    ) -> Result<OkCase, ErrCase> {
        // I have received FinishTransaction
        // return received FinishTransaction packet_meta_data
        unimplemented!()
    }
    */

    pub fn route(&self, packet_meta_data: PacketMetaData) -> Result<OkCase, ErrCase> {
        match packet_meta_data.spec_state {
            SpecState::Normal => {
                let is_broadcast = packet_meta_data.is_destination_identifyer_reached(
                    &DeviceIdentifyer(BROADCAST_RESERVED_IDENTIFYER),
                );
                match is_broadcast {
                    true => self.handle_broadcast(packet_meta_data),
                    false => self.handle_normal(packet_meta_data),
                }
            }
            SpecState::PingPacket => self.handle_ping(packet_meta_data),
            SpecState::PongPacket => self.handle_pong(packet_meta_data),
            _ => Ok(OkCase::Handled),
            // SpecState::SendTransaction => self.handle_send_transaction(packet_meta_data),
            // SpecState::AcceptTransaction => self.handle_accept_transaction(packet_meta_data),
            // SpecState::InitTransaction => self.handle_init_transaction(packet_meta_data),
            // SpecState::FinishTransaction => self.handle_finish_transaction(packet_meta_data),
        }
    }
}
