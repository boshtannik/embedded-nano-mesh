#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use heapless::String;
use panic_halt as _;

mod config;
mod millis;
mod serial;
mod transciever;

use millis::{millis, millis_init, ms};

use transciever::{DeviceIdentifyer, LifeTimeType, Transciever, TranscieverString};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    millis_init(dp.TC0);

    let pins = arduino_hal::pins!(dp);

    let mut led_pin = pins.d13.into_output();
    let mut last_blink_time = millis();

    serial::init(default_serial!(dp, pins, 57600));

    unsafe { avr_device::interrupt::enable() };

    let mut transciever = Transciever::new(DeviceIdentifyer(2), 230 as ms);

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
        if now_time > (last_blink_time + 250 as ms) {
            led_pin.toggle();
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
                Err(transciever::TranscieverError::TryAgainLater) => {
                    serial_println!("Too much packets, Transciever says try later");
                }
            };

            packet_counter = packet_counter.overflowing_add(1).0;
        }
    }
}
