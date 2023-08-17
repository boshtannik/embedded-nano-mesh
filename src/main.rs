#![no_std]
#![no_main]

use arduino_hal::default_serial;
use panic_halt as _;

mod config;
mod serial;
mod transciever;

use transciever::{DeviceIdentifyer, Transciever, TranscieverString};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    serial::init(default_serial!(dp, pins, 57600));

    let mut transciever = Transciever::new(DeviceIdentifyer(1));
    transciever
        .send(
            TranscieverString::from("Hello world").into_bytes(),
            DeviceIdentifyer(2),
        )
        .unwrap_or_else(|_| {});
    loop {
        transciever.update();
        if let Some(_) = transciever.receive() {
            serial_println!("data been received");
        }
    }
}
