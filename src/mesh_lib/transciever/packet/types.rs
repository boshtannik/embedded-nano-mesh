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

/// `FlagsType` -         Is just typical alias to integer like type, with next fields being defined:
///
/// `IgnoreDuplication` - Flag, that forces device to filter out duplicated or in other words echoed packet from ether.
///
/// Echoed packet - means, that packet have been retransmitted by the intermediate node of the
/// network and have same packet_id and sender's device identifyer.
///
/// Now I about to implement special flag `IgnoreDuplication` flag handling. If `IgnoreDuplication` flag is set, it
/// says to every receiving device to record meta information of packet, and further ignore all same echoed
/// packets that are caught from the ether for some period of time, which is defined in
/// `RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD` constant of `transciever/config.rs` file.
///
/// NOTE: Just for note. I have idea for special sending mode to guarantee, that receiving device
/// act on sent message only once. It is `Transaction`. `Transaction` will consist of four steps.
///
/// `Transaction` is needed to make sure, that receiving device will act on message  only `ONCE`.
/// `Transaction` may be useful in urgent use case, if multiple devices need to act, and keep knowledge
/// of the same state after the action.
/// *   For example: It is needed to switch on the water valve.
///     In the regular use case sending packet with command to switch on tha water valve and send
///     back the response that tells that the valve has been switched on - might be useful, except
///     one thin situation... The command message, with the request to answer back is sent, and the
///     receiving device has received the command, and have answered back, but the answer message
///     might not get back to the sender. It creates dangerous situation, in which sender's side will not
///     know that water valve stays open . To fix this - both devices should guaranteed to act in the
///     strict manner, or make changes back, in case if something goes wrong - this all is called the `Transaction`.
///
/// So the `Transaction` is made of next several steps:
/// Each step - is the special packet, with special flag being set:
///
///     1 `SendTransaction`,        -   Seinding device says to receiving device, that the sending device
///                                     is offering transaction.
///
///     2 `AcceptTransaction`,      -   Receiving device says back to sending device, that receiving device, accepts the transaction.
///
///     3 `InitiateTransaction`,    -   Sending device says back to receiving device, that transaction initiator now knows, that receiving
///                                     device is ready for transaction being started and waits for transaction to be completed.
///
///     4 `FinishTransaction`.      -   Receiving device says to sending device, that it's have
///                                     the transaction done.
///
///     TODO: Unify words message and packet, to void misinterpritations.
///     TODO: Make devices names, Sender - Device A, Receiver - Device B.
///     
///     ---------------------                                                                                                           ---------------------
///     |                   |                                                                                                           |                   |
///     |    Transaction    |                                                                                                           |    Transaction    |
///     |       sender      |                                                                                                           |      receiver     |
///     |                   |                                                                                                           |                   |
///     ---------------------                                                                                                           ---------------------
///               |                                                                                                                               |
///               |                                                                                                                               |
///               |                                                 1. `SendTransaction`.                                                         |
///               |    In that moment of time, only sending device knows, about act, that is needed to be done. So sender changing it's state     |
///               |    to the new one, and waits, that the receiver will change it's state too. In case, if no next transaction packet will       |
///               |    be received back - will roll back the previous state. So to make the transaction, the sender sends the information about   |
///               |    the transaction to the receiver within `SendTransaction` flag in packet.                                                   |
///               |                                                                                                                               |
///               |    -------------------------------------------------------------------------------------------------------------------->      |
///               |                                                                                                                               |
///               |                                                 2. `AcceptTransaction`.                                                       |
///               |    In that moment of time, sneding device, has no information about the receiving device might have the message caught, or    |
///               |    might have not the message caught yet. And the receiver device now, has changed it's state to the new one. So receiver     |
///               |    informs the sender, about transaction being accepted.                                                                      |
///               |    So the receiver is sending that information back to sender within `AcceptTransaction` flag in packet.                      |
///               |                                                                                                                               |
///               |    <--------------------------------------------------------------------------------------------------------------------      |
///               |                                                                                                                               |
///               |                                                 3. `InitiateTransaction`.                                                     |
///               |    In that moment of time, the sender knows, that it keeps it's new state, but the receiver needs to know if the new          |
///               |    state needs to be kept by the receiver or rolled back to previous one. So to keep new state by the receiver,               |
///               |    the sender sends `InitiateTransaction` flag within the packet.                                                             |
///               |                                                                                                                               |
///               |    -------------------------------------------------------------------------------------------------------------------->      |
///               |                                                                                                                               |
///               |                                                 4. `FinishTransaction`.                                                       |
///               |    In that moment of time, the sender does not know yet, if the receiver keeps it's new state, or will drop it back.          |
///               |    So to fix that, the receiver sends packet with `FinishTransaction` flag being set.                                         |
///               |                                                                                                                               |
///               |    <--------------------------------------------------------------------------------------------------------------------      |
///               |                                                                                                                               |
///               |                                                                                                                               |
///               |                                                                                                                               |
///
///
/// The meaning of those flags - is to provide packed information about the purpose of the packet.

pub type FlagsType = u8;
pub const IGNORE_DUPLICATIONS_FLAG: FlagsType = 0b10000000;

pub type LifeTimeType = u8;
pub type IdType = u8;

const ADDRESS_TYPE_SIZE: usize = size_of::<AddressType>();
pub const DEVICE_IDENTIFYER_TYPE_SIZE: usize = ADDRESS_TYPE_SIZE;
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

/*
1. `SendTransaction`.
In that moment of time, only sending device knows, about act, that is needed to be done. So sender changing it's state
to the new one, and waits, that the receiver will change it's state too. In case, if no next transaction packet will
be received back - will roll back the previous state. So to make the transaction, the sender sends the information about
the transaction to the receiver within `SendTransaction` flag in packet.

2. `AcceptTransaction`.
In that moment of time, sneding device, has no information about the receiving device might have the message caught, or
might have not the message caught yet. And the receiver device now, has changed it's state to the new one. So receiver
informs the sender, about transaction being accepted.
So the receiver is sending that information back to sender within `AcceptTransaction` flag in packet.

3. `InitiateTransaction`.
In that moment of time, the sender knows, that it keeps new state, but the receiver needs to know if the new
state needs to be kept by the receiver or rolled back to previous one. So to keep new state by the receiver,
the sender sends `InitiateTransaction` flag within the packet.

4. `FinishTransaction`.
In that moment of time, the sender does not know yet, if the receiver keeps it's new state, or will drop it back.
So to fix that, the receiver sends packet with `FinishTransaction` flag being set.
*/
