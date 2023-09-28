#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use mesh_lib::{DeviceIdentifyer, LifeTimeType, TranscieverConfig};
use panic_halt as _;

mod mesh_lib;

use heapless::String;
use mesh_lib::millis::{millis, ms};

use mesh_lib::TranscieverString;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut transciever = mesh_lib::init_transciever(TranscieverConfig {
        device_identifyer: DeviceIdentifyer(1),
        listen_period: 360 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_timer: dp.TC0,
    });

    let mut last_send_time: ms = millis();
    let mut packet_counter: u32 = 0;
    let mut led_pin = pins.d13.into_output();

    loop {
        let _ = transciever.update();

        /*
        if let Some(message) = transciever.receive() {
            led_pin.toggle();
            serial_println!("\nMsg");
            for byte in message.data {
                serial_write_byte!(byte).unwrap_or({});
            }
        }
        */

        let now_time = millis();
        if now_time > (last_send_time + 1000 as ms) {
            led_pin.toggle();
            last_send_time = now_time;

            let packet_num: String<10> = String::from(packet_counter);

            let mut message = TranscieverString::from("Packet #: ");

            message.push_str(&packet_num).unwrap();

            while message.len() != message.capacity() {
                message.push('\0').unwrap_or_else(|_| {});
            }

            if let Ok(_) = transciever.send_with_transaction(
                message.into_bytes(),
                DeviceIdentifyer(1),
                4 as LifeTimeType,
                true,
                2000 as ms,
            ) {
                led_pin.toggle();
                serial_println!("Transaction done!");
            } else {
                serial_println!("Transaction not done!");
            }
            packet_counter = packet_counter.overflowing_add(1).0;
        }
    }
}
