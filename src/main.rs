#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use panic_halt as _;

mod config;
mod millis;
mod serial;
mod transciever;

use millis::{millis_init, ms};

use transciever::{DeviceIdentifyer, Transciever, TranscieverString};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    millis_init(dp.TC0);

    let pins = arduino_hal::pins!(dp);

    unsafe { avr_device::interrupt::enable() };

    serial::init(default_serial!(dp, pins, 57600));

    let mut transciever = Transciever::new(DeviceIdentifyer(2), 1000 as ms);
    transciever
        .send(
            TranscieverString::from("Hello world").into_bytes(),
            DeviceIdentifyer(2),
        )
        .unwrap_or_else(|_| {});
    loop {
        transciever.update();
        if let Some(received_message) = transciever.receive() {
            for byte in received_message.iter() {
                serial_write_byte!(*byte).unwrap();
            }
            serial_write_byte!(b'\r').unwrap();
            serial_write_byte!(b'\n').unwrap();
        }
    }
}
