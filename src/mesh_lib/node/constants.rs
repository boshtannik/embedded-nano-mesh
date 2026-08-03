use super::ms;

const SECOND: ms = 1000;

pub const PACKET_QUEUE_SIZE: usize = 5;

/// Start byte of packet. The device will recognize
/// packets by this byte.
pub const PACKET_START_BYTE: u8 = b'x';

/// Start bytes count of packet.
pub const PACKET_START_BYTES_COUNT: usize = 3;

/// Strategy used by the duplicate filter when its registration table
/// is full.
///
/// Configurable at compile time via the environment variable
/// `ENM_RECEIVER_FILTER_OVERFLOW_STRATEGY` or the `.env` file
/// (`evict_oldest` | `drop_new`).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DuplicateFilterOverflowStrategy {
    /// Drop the oldest entry to make room for the new packet.
    EvictOldest,
    /// Reject the new packet and keep the existing entries.
    DropNew,
}

/// Strategy used by the duplicate filter when a duplicate of an
/// already-known packet is detected.
///
/// Configurable at compile time via the environment variable
/// `ENM_RECEIVER_FILTER_DUPLICATE_STRATEGY` or the `.env` file
/// (`ignore` | `double_period`).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DuplicateFilterDuplicateStrategy {
    /// Ignore the duplicate packet and leave the ignore period unchanged.
    Ignore,
    /// Ignore the duplicate packet and double the period during which
    /// duplicates of any packet are ignored. This is a simple anti-flood
    /// backoff: the busier the network, the longer echoes are suppressed.
    DoublePeriod,
}

pub(crate) const fn parse_usize(value: &str) -> usize {
    let bytes = value.as_bytes();
    let mut result: usize = 0;
    let mut i = 0;
    while i < bytes.len() {
        let digit = bytes[i];
        if digit < b'0' || digit > b'9' {
            panic!("ENM config value is not a number");
        }
        result = result * 10 + (digit - b'0') as usize;
        i += 1;
    }
    if result == 0 {
        panic!("ENM config value must be positive");
    }
    result
}

const fn str_eq_bytes(value: &str, expected: &[u8]) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != expected.len() {
        return false;
    }
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != expected[i] {
            return false;
        }
        i += 1;
    }
    true
}

const fn parse_overflow_strategy(value: &str) -> DuplicateFilterOverflowStrategy {
    if str_eq_bytes(value, b"evict_oldest") {
        return DuplicateFilterOverflowStrategy::EvictOldest;
    }
    if str_eq_bytes(value, b"drop_new") {
        return DuplicateFilterOverflowStrategy::DropNew;
    }
    panic!("invalid ENM_RECEIVER_FILTER_OVERFLOW_STRATEGY value");
}

const fn parse_duplicate_strategy(value: &str) -> DuplicateFilterDuplicateStrategy {
    if str_eq_bytes(value, b"ignore") {
        return DuplicateFilterDuplicateStrategy::Ignore;
    }
    if str_eq_bytes(value, b"double_period") {
        return DuplicateFilterDuplicateStrategy::DoublePeriod;
    }
    panic!("invalid ENM_RECEIVER_FILTER_DUPLICATE_STRATEGY value");
}

/// Count of filter's table, that holds reocords for packets, that
/// need to be ignored.
///
/// Configurable at compile time via the environment variable
/// `ENM_RECEIVER_FILTER_REGISTRATION_SIZE` or the `.env` file.
pub const RECEIVER_FILTER_REGISTRATION_SIZE: usize =
    match option_env!("ENM_RECEIVER_FILTER_REGISTRATION_SIZE") {
        Some(value) => parse_usize(value),
        None => 8,
    };

/// Perid of time, during which duplicated packets will be ignored.
///
/// Configurable at compile time via the environment variable
/// `ENM_RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD` or the `.env` file.
pub const RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD: ms =
    match option_env!("ENM_RECEIVER_FILTER_DUPLICATE_IGNORE_PERIOD") {
        Some(value) => parse_usize(value) as ms,
        None => SECOND,
    };

/// Strategy applied when the duplicate filter's registration table
/// overflows.
///
/// Configurable at compile time via the environment variable
/// `ENM_RECEIVER_FILTER_OVERFLOW_STRATEGY` or the `.env` file.
pub const RECEIVER_FILTER_OVERFLOW_STRATEGY: DuplicateFilterOverflowStrategy =
    match option_env!("ENM_RECEIVER_FILTER_OVERFLOW_STRATEGY") {
        Some(value) => parse_overflow_strategy(value),
        None => DuplicateFilterOverflowStrategy::EvictOldest,
    };

/// Strategy applied when a duplicate of an already-known packet is
/// detected.
///
/// Configurable at compile time via the environment variable
/// `ENM_RECEIVER_FILTER_DUPLICATE_STRATEGY` or the `.env` file.
pub const RECEIVER_FILTER_DUPLICATE_STRATEGY: DuplicateFilterDuplicateStrategy =
    match option_env!("ENM_RECEIVER_FILTER_DUPLICATE_STRATEGY") {
        Some(value) => parse_duplicate_strategy(value),
        None => DuplicateFilterDuplicateStrategy::DoublePeriod,
    };

/// Size of read buffer from serial interface for receiver.
pub const RECEIVER_READER_BUFFER_SIZE: usize = 1;
