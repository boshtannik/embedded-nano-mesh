mod atmega328p;

#[allow(non_camel_case_types)]
pub type ms = u32;

pub use atmega328p::{millis, millis_init};
