#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use mesh_lib::{DeviceIdentifyer, LifeTimeType, NodeConfig};
use panic_halt as _;

mod mesh_lib;

use heapless::String;
use mesh_lib::millis::{millis, ms};

use mesh_lib::NodeString;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut mesh_node = mesh_lib::init_node(NodeConfig {
        device_identifyer: DeviceIdentifyer(2),
        listen_period: 360 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_timer: dp.TC0,
    });

    let mut last_send_time: ms = millis();
    let mut packet_counter: u32 = 0;
    let mut led_pin = pins.d13.into_output();

    loop {
        let _ = mesh_node.update();

        if let Some(message) = mesh_node.receive() {
            led_pin.toggle();
            serial_println!("\nMsg");
            for byte in message.data {
                serial_write_byte!(byte).unwrap_or({});
            }
        }

        let now_time = millis();
        if now_time > (last_send_time + 1000 as ms) {
            last_send_time = now_time;

            let packet_num: String<10> = String::from(packet_counter);

            let mut message = NodeString::from("Packet #: ");

            message.push_str(&packet_num).unwrap();

            while message.len() != message.capacity() {
                message.push('\0').unwrap_or_else(|_| {});
            }

            if let Ok(_) = mesh_node.send_with_transaction(
                message.into_bytes(),
                DeviceIdentifyer(1),
                10 as LifeTimeType,
                true,
                5000 as ms,
            ) {
                led_pin.toggle();
                serial_println!("Transaction done!");
            } else {
                serial_println!("Transaction failed!");
            }
            packet_counter = packet_counter.overflowing_add(1).0;
        }
    }
}
