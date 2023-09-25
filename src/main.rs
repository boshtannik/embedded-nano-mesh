#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use mesh_lib::transciever::BROADCAST_RESERVED_IDENTIFYER;
use mesh_lib::{DeviceIdentifyer, TranscieverConfig, TranscieverError};
use panic_halt as _;

mod mesh_lib;

use heapless::String;
use mesh_lib::millis::{millis, ms};

use mesh_lib::{LifeTimeType, TranscieverString};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut transciever = mesh_lib::init_transciever(TranscieverConfig {
        device_identifyer: DeviceIdentifyer(2),
        listen_period: 160 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_timer: dp.TC0,
    });

    let mut last_send_time: ms = millis();
    let mut packet_counter: u32 = 0;
    let mut led_pin = pins.d13.into_output();

    loop {
        transciever.update();
        if let Some(received_message) = transciever.receive() {
            led_pin.toggle();
            serial_println!("Caught packet back: ");
            for byte in received_message.iter() {
                serial_write_byte!(*byte).unwrap();
            }
            serial_println!("");
        }

        let now_time = millis();
        if now_time > (last_send_time + 1000 as ms) {
            led_pin.toggle();
            last_send_time = now_time;

            let packet_num: String<20> = String::from(packet_counter);

            let mut message = TranscieverString::from("Packet #: ");

            message.push_str(&packet_num).unwrap();

            while message.len() != message.capacity() {
                message.push('\0').unwrap_or_else(|_| {});
            }

            match transciever.send(
                message.into_bytes(),
                DeviceIdentifyer(BROADCAST_RESERVED_IDENTIFYER),
                LifeTimeType::from(3),
                true,
            ) {
                Ok(_) => {}
                Err(TranscieverError::SendingQueueIsFull) => {}
            };

            packet_counter = packet_counter.overflowing_add(1).0;
        }
    }
}
