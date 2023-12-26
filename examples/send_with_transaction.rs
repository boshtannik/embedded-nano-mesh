#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate embedded_nano_mesh;

use arduino_hal::default_serial;
use embedded_nano_mesh::{
    AddressType, LifeTimeType, Node, NodeConfig, NodeString, SpecialSendError,
};
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

    match mesh_node.send_with_transaction::<Atmega328pTime, ArduinoNanoSerial>(
        NodeString::from("This is the message to be sent").into_bytes(),
        2 as AddressType,
        10 as LifeTimeType,
        true,
        3000 as ms,
    ) {
        Ok(()) => {
            ufmt::uwriteln!(&mut ArduinoNanoSerial::default(), "Packet sent").unwrap();
        }
        Err(SpecialSendError::SendingQueueIsFull) => {
            ufmt::uwriteln!(&mut ArduinoNanoSerial::default(), "SendingQueueIsFull").unwrap();
        }
        Err(SpecialSendError::MulticastAddressForbidden) => {
            ufmt::uwriteln!(
                &mut ArduinoNanoSerial::default(),
                "MulticastAddressForbidden"
            )
            .unwrap();
        }
        Err(SpecialSendError::Timeout) => {
            ufmt::uwriteln!(&mut ArduinoNanoSerial::default(), "Timeout").unwrap();
        }
    }

    loop {
        let _ = mesh_node.update::<Atmega328pTime, ArduinoNanoSerial>();
    }
}
