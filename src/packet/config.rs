use heapless::String;
pub const CONTENT_SIZE: usize = 64;

pub type String64 = String<64>;

pub type AddressType = u8;
pub type ChecksumType = u8;
pub type FlagsType = u128;
