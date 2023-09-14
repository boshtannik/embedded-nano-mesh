#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use nano_mesh::{DeviceIdentifyer, TranscieverConfig, TranscieverError};
use panic_halt as _;

mod mesh_lib;

use heapless::String;
use mesh_lib::millis::{millis, ms};

use nano_mesh::{LifeTimeType, TranscieverString};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut transciever = nano_mesh::init_transciever(TranscieverConfig {
        device_identifyer: DeviceIdentifyer(2),
        listen_period: 50 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_timer: dp.TC0,
    });

    let mut last_blink_time = millis();

    let mut packet_counter: u32 = 0;

    loop {
        transciever.update();
        if let Some(received_message) = transciever.receive() {
            serial_println!("Message has reached it's destination!");
            for byte in received_message.iter() {
                serial_write_byte!(*byte).unwrap();
            }
            serial_write_byte!(b'\r').unwrap();
            serial_write_byte!(b'\n').unwrap();
        }

        let now_time = millis();
        if now_time > (last_blink_time + 150 as ms) {
            last_blink_time = now_time;

            let packet_num: String<20> = String::from(packet_counter);

            let mut message = TranscieverString::from("Packet #: ");

            message.push_str(&packet_num).unwrap();

            while message.len() != message.capacity() {
                message.push('\0').unwrap_or_else(|_| {});
            }
            match transciever.send(
                message.into_bytes(),
                DeviceIdentifyer(2),
                LifeTimeType::from(3),
            ) {
                Ok(_) => {}
                Err(TranscieverError::TryAgainLater) => {
                    serial_println!("Too much packets, Transciever says try later");
                }
            };

            packet_counter = packet_counter.overflowing_add(1).0;
        }
    }
}
