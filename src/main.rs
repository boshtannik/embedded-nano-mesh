#![no_std]
#![no_main]

use arduino_hal::default_serial;
use panic_halt as _;

mod config;
mod serial;
mod transciever;

use transciever::{DeviceIdentifyer, PacketString, Transciever};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    serial::init(default_serial!(dp, pins, 57600));

    let mut transciever = Transciever::new(DeviceIdentifyer(1));
    transciever
        .send_message(PacketString::from("Hello world"), DeviceIdentifyer(2))
        .unwrap_or_else(|_| {});
    loop {
        transciever.update();
    }
}
