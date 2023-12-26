#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate embedded_nano_mesh;

use arduino_hal::default_serial;
use embedded_nano_mesh::{AddressType, LifeTimeType, Node, NodeConfig, NodeString, SendError};
use panic_halt as _;

use platform_millis_arduino_nano::{init_timer, ms, Atmega328pTime};
use platform_serial_arduino_nano::{init_serial, ArduinoNanoSerial};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    init_timer(dp.TC0);
    init_serial(default_serial!(dp, pins, 9600));

    let mut mesh_node = Node::new(NodeConfig {
        device_address: 1 as AddressType,
        listen_period: 150 as ms,
    });

    match mesh_node.send(
        NodeString::from("This is the message to be sent").into_bytes(),
        2 as AddressType,
        10 as LifeTimeType,
        true,
    ) {
        Ok(()) => {
            ufmt::uwriteln!(&mut ArduinoNanoSerial::default(), "Packet sent").unwrap();
        }
        Err(SendError::SendingQueueIsFull) => {
            ufmt::uwriteln!(&mut ArduinoNanoSerial::default(), "SendingQueueIsFull").unwrap();
        }
    }

    loop {
        let _ = mesh_node.update::<Atmega328pTime, ArduinoNanoSerial>();
    }
}
