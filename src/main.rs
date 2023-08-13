#![no_std]
#![no_main]

use arduino_hal::default_serial;
use packet::{DeviceIdentifyer, PacketString};
use panic_halt as _;

mod config;
mod packet;
mod serial;
mod transciever;

use transciever::Transciever;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    serial::init(default_serial!(dp, pins, 57600));

    let mut transciever = Transciever::new(DeviceIdentifyer(1));
    /*transciever
        .send_message(String64::from("Hello world"), DeviceIdentifyer(2))
        .unwrap_or_else(|_| serial_println!("Error of sending message over transciever"));
    */
    loop {
        // transciever.update();
    }
}
