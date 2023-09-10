#![no_std]
#![feature(abi_avr_interrupt)]

mod mesh_lib;

use avr_device::atmega328p::TC0;

pub use mesh_lib::millis::{millis, millis_init, ms};
pub use mesh_lib::serial::Usart;
pub use mesh_lib::transciever::{DeviceIdentifyer, LifeTimeType, Transciever, TranscieverString};

pub struct TranscieverConfig {
    device_identifyer: DeviceIdentifyer,
    listen_period: ms,
    usart: Usart,
    millis_timer: TC0,
}

pub fn init_transciever(config: TranscieverConfig) -> Transciever {
    millis_init(config.millis_timer);
    mesh_lib::serial::init(config.usart);
    unsafe { avr_device::interrupt::enable() };
    Transciever::new(config.device_identifyer, config.listen_period)
}
