#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use mesh_lib::{init_node, AddressType, LifeTimeType, NodeConfig};
use panic_halt as _;

mod mesh_lib;
mod platform_specific_millis_timer;

use mesh_lib::millis::ms;

use mesh_lib::NodeString;
use ufmt::uwrite;

use mesh_lib::millis::PlatformTime;
use platform_specific_millis_timer::init_timer;
use platform_specific_millis_timer::AvrTime;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    init_timer(dp.TC0);

    let platform_specific_millis_timer = AvrTime;

    let mut mesh_node = init_node(NodeConfig {
        device_identifier: 1 as AddressType,
        listen_period: 150 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_timer: &platform_specific_millis_timer,
    });

    let mut last_send_time: ms = platform_specific_millis_timer.millis();
    let mut now_time: ms;
    let mut packet_counter: u32 = 0;

    loop {
        let _ = mesh_node.update();

        now_time = platform_specific_millis_timer.millis();

        if now_time > (last_send_time + 310 as ms) {
            let mut message = NodeString::new();
            uwrite!(&mut message, "Packet #: {}", packet_counter).unwrap();

            mesh_node
                .send_with_transaction(
                    message.clone().into_bytes(),
                    2 as AddressType,
                    10 as LifeTimeType,
                    true,
                    3000 as ms,
                )
                .unwrap_or_else(|_| serial_debug!("Transaction failed"));

            last_send_time = now_time;
            packet_counter = packet_counter.overflowing_add(1).0;
        }
    }
}
