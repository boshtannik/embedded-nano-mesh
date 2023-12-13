use platform_millis_atmega328p::ms;

pub const QUEUE_SIZE: usize = 5;
pub const PACKET_START_BYTES_COUNT: usize = 3;
pub const PACKET_START_BYTE: u8 = b'x';

pub const RECEIVER_FILTER_REGISTRATION_SIZE: usize = 8;
pub const RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD: ms = 1000;
