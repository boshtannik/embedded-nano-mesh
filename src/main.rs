#![no_std]
#![no_main]

use arduino_hal::default_serial;
use packet::DeviceAddress;
use panic_halt as _;

mod config;
mod packet;
mod serial;
mod transciever;

use transciever::Transciever;

// Multiple layers networking
// 1 - Layer of data conveyuor.
//      1.1 - Sending queue.
//      1.2 - Receiving queue.
//      1.3 - Received messages iterator
//      1.4 - Sent messages method
//
// 2 - Layer of data routing
//      2.1 - Transit packets queue

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    serial::init(default_serial!(dp, pins, 57600));

    let mut transciever_ins = Transciever::new(DeviceAddress(1));
    // transciever_ins.send_message(String::from("Hello world"), DeviceAddress(2));
    serial_println!("Second use of serial println").unwrap();

    loop {
        transciever_ins.update();
        for received_message in transciever_ins.received_messages() {
            for byte in received_message.as_bytes() {
                serial_write_byte!(*byte).unwrap();
            }
        }
    }
}
