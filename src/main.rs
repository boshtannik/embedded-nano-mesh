#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use mesh_lib::{AddressType, LifeTimeType, Node, NodeString};
use panic_halt as _;

mod mesh_lib;

use ufmt::uwrite;

use platform_millis_arduino_nano::{init_timer, ms, Atmega328pTime, PlatformTime};
use platform_serial_arduino_nano::{init_serial, ArduinoNanoSerial};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    init_timer(dp.TC0);
    init_serial(default_serial!(dp, pins, 9600));

    let mut mesh_node = Node::new(1 as AddressType, 150 as ms);

    let mut last_send_time: ms = Atmega328pTime::millis();
    let mut now_time: ms;
    let mut packet_counter: u32 = 0;

    loop {
        let _ = mesh_node.update::<Atmega328pTime, ArduinoNanoSerial>();

        now_time = Atmega328pTime::millis();

        if now_time > (last_send_time + 310 as ms) {
            let mut message = NodeString::new();
            uwrite!(&mut message, "Packet #: {}", packet_counter).unwrap();

            match mesh_node.send_with_transaction::<Atmega328pTime, ArduinoNanoSerial>(
                message.clone().into_bytes(),
                2 as AddressType,
                10 as LifeTimeType,
                true,
                3000 as ms,
            ) {
                Ok(_) => uwrite!(&mut ArduinoNanoSerial::default(), "- Transaction done!\n")
                    .unwrap_or_else(|_| {}),
                Err(_) => uwrite!(&mut ArduinoNanoSerial::default(), "- Transaction falied!\n")
                    .unwrap_or_else(|_| {}),
            }

            last_send_time = now_time;
            packet_counter = packet_counter.overflowing_add(1).0;
        }
    }
}
