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

/*
* Done - Problem with non working shared queue. - Done
* 1 - Problem with variable size payload in packet.
*       (Temporary solve) - Fill the rest of free space by 0 bytes.
*       This place will be marked as ===Place of temporary solve 1===
* 2 - Problem with unknown packet size (hardcode for now). - Need to be
*       calculated at compilation time
* 3 - Packet is living forever. Lifetime shall be added. Think about
*       reduce packet jamming over the ether
*/

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    millis_init(dp.TC0);

    let pins = arduino_hal::pins!(dp);

    serial::init(default_serial!(dp, pins, 57600));

    unsafe { avr_device::interrupt::enable() };

    let mut transciever = Transciever::new(DeviceIdentifyer(1), 1000 as ms);

    let mut sending_string: TranscieverString = TranscieverString::new();

    /*
    transciever
        .send(sending_string.into_bytes(), DeviceIdentifyer(2))
        .unwrap_or_else(|_| {});
    */

    loop {
        transciever.update();
        if let Some(received_message) = transciever.receive() {
            serial_println!("Message received back!");
            for byte in received_message.iter() {
                serial_write_byte!(*byte).unwrap();
            }
            serial_write_byte!(b'\r').unwrap();
            serial_write_byte!(b'\n').unwrap();
        }
    }
}
