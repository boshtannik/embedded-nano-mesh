#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use mesh_lib::{init_node, DeviceIdentifier, LifeTimeType, NodeConfig};
use panic_halt as _;

mod mesh_lib;

use mesh_lib::millis::{millis, ms};

use mesh_lib::NodeString;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut mesh_node = init_node(NodeConfig {
        device_identifier: DeviceIdentifier(2),
        listen_period: 150 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_timer: dp.TC0,
    });

    let mut led_pin = pins.d13.into_output();

    // let mut last_send_time: ms = millis();
    // let mut now_time: ms;
    // let mut packet_counter: u32 = 0;

    loop {
        let _ = mesh_node.update();

        if let Some(_) = mesh_node.receive() {
            led_pin.toggle();
        }
        /*

        now_time = millis();
        if now_time > (last_send_time + 210 as ms) {
            let mut message = NodeString::new();
            uwrite!(&mut message, "Packet #: {}", packet_counter).unwrap();

            mesh_node
                .send(
                    message.into_bytes(),
                    DeviceIdentifier(2),
                    10 as LifeTimeType,
                    true,
                )
                .unwrap_or_else(|_| serial_println!("Not sent!"));

            last_send_time = now_time;
            packet_counter = packet_counter.overflowing_add(1).0;
        }
        */
    }
}
