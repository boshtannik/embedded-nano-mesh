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

    /* I guess to make the device constrained in time.
     * By creating speak / listen ratio.
     * It might be like 1 : 10 or 1 : 100
     * meaning that in one second, the device
     * will be speaking into the ether 100ms of time, and will be listening 900ms of it.
     * Or will be speaking 10 ms, and 990 ms accordingly. That shall reduce packet collision.
     * It is just the assumption, which i have not tested yet, and yet shall be tested.
     */

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
