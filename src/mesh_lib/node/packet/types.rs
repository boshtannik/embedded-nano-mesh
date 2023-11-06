use core::mem::size_of;

use heapless::Vec;

use super::{
    config::{CONTENT_SIZE, PACKET_BYTES_COUNT},
    FromBytes,
};

pub type PacketDataBytes = Vec<u8, CONTENT_SIZE>;

pub type PacketSerializedBytes = Vec<u8, PACKET_BYTES_COUNT>;

pub type AddressType = u8;
pub type ChecksumType = u8;

pub type FlagsType = u8;
/// This flag make the device, which have packet caught, to ignore
/// same packets, which were re-transmitted from other devices.
pub const IGNORE_DUPLICATIONS_FLAG: FlagsType = 0b10000000;

/// This flag says, that the receiving device, should respond
/// with packet with `PONG_FLAG` being set.
pub const PING_FLAG: FlagsType = 0b01000000;

/// This flag says, that the packet, is made to
/// provide the answer to the device, which is waiting
/// for the answer.
pub const PONG_FLAG: FlagsType = 0b00100000;

/// This flag tells, that this packet is made by
/// transaction sender device, is to create, and
/// send the transaction to the transaction
/// reponding device.
pub const SEND_TRANSACTION_FLAG: FlagsType = 0b00010000;

/// This flag tells, that this packet is made by
/// transaction responding device, is to accept, and
/// continue the transaction to the transaction
/// sender's device.
pub const ACCEPT_TRANSACTION_FLAG: FlagsType = 0b00001000;

/// This flag tells, that this packet is made by
/// transaction sneder device, is to initiate, and
/// continue the transaction to the transaction
/// reponding device.
pub const INITIATE_TRANSACTION_FLAG: FlagsType = 0b00000100;

/// This flag tells, that the packet, which contains
/// this flag - is the last packet in the transaction
/// sqeuence, and tells, that transaction receiver
/// device is fully accepted the transaction, and informs
/// transaction sender's device about that.
pub const FINISH_TRANSACTION_FLAG: FlagsType = 0b00000010;

// This flag is not used yet.
//pub const NOT_USED_FLAG: FlagsType = 0b00000001;

pub type LifeTimeType = u8;
pub type IdType = u8;

const ADDRESS_TYPE_SIZE: usize = size_of::<AddressType>();
pub const DEVICE_IDENTIFIER_TYPE_SIZE: usize = ADDRESS_TYPE_SIZE;
pub const ID_TYPE_SIZE: usize = size_of::<IdType>();
pub const LIFETIME_TYPE_SIZE: usize = size_of::<LifeTimeType>();
pub const FLAGS_TYPE_SIZE: usize = size_of::<FlagsType>();
pub const DATA_LENGTH_TYPE_SIZE: usize = size_of::<usize>();
pub const DATA_TYPE_SIZE: usize = CONTENT_SIZE;
pub const CHECKSUM_TYPE_SIZE: usize = size_of::<ChecksumType>();

impl FromBytes<ADDRESS_TYPE_SIZE> for AddressType {
    fn from_be_bytes(bytes: [u8; ADDRESS_TYPE_SIZE]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

impl FromBytes<DATA_LENGTH_TYPE_SIZE> for usize {
    fn from_be_bytes(bytes: [u8; DATA_LENGTH_TYPE_SIZE]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum PacketState {
    Normal,
    Ping,
    Pong,
    SendTransaction,
    AcceptTransaction,
    InitTransaction,
    FinishTransaction,
}

#[derive(Clone, PartialEq, Eq)]
pub struct DeviceIdentifier(pub AddressType);
