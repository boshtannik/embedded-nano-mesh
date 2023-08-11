#![no_std]
#![no_main]

use core::cell::Cell;
use std::process::Output;

use arduino_hal::{
    clock::MHz16,
    default_serial,
    hal::{
        port::{PD0, PD1},
        Atmega,
    },
    pac::USART0,
    port::mode::Input,
};
use heapless::String;
use packet::DeviceAddress;
use panic_halt as _;

mod config;
mod packet;
mod transciever;
use packet::String64;
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

type SerialType = Cell<
    avr_hal_generic::usart::Usart<
        Atmega,
        USART0,
        avr_hal_generic::port::Pin<Input, PD0>,
        avr_hal_generic::port::Pin<Output, PD1>,
        MHz16,
    >,
>;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let led_pin = pins.d13.into_output();

    let mut serial = default_serial!(dp, pins, 57600);
    let mut celled_serial = Cell::new(serial);

    let mut transciever_ins = Transciever::new(DeviceAddress(1), Cell::clone(&celled_serial));
    // transciever_ins.send_message(String::from("Hello world"), DeviceAddress(2));

    let stringa = String64::from("Hello world");

    loop {}
}
