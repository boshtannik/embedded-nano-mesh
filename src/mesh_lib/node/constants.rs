use platform_millis::ms;

pub const PACKET_QUEUE_SIZE: usize = 5;

pub const PACKET_META_DATA_QUEUE_SIZE: usize = 10;

/// Start byte of packet. The device will recognize
/// packets by this byte.
pub const PACKET_START_BYTE: u8 = b'x';

/// Start bytes count of packet.
pub const PACKET_START_BYTES_COUNT: usize = 3;

/// Count of filter's table, that holds reocords for packets, that
/// need to be ignored.
pub const RECEIVER_FILTER_REGISTRATION_SIZE: usize = 8;

/// Perid of time, during which duplicated packets will be ignored.
pub const RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD: ms = 1000;
