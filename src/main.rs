#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
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
        listen_period: 150 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_timer: dp.TC0,
    });

    let mut last_send_time: ms = millis();

    let mut packet_counter: u32 = 0;

    loop {
        transciever.update();
        if let Some(received_message) = transciever.receive() {
            serial_println!("Message has reached it's destination!");
            for byte in received_message.iter() {
                serial_write_byte!(*byte).unwrap();
            }
            serial_println!("");
        }

        let now_time = millis();
        if now_time > (last_send_time + 360 as ms) {
            last_send_time = now_time;

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

/*
// On broadcasting, all devices will receive that message, and will be able to react on it,
// as if the message was sent exactly to the device.
// The broadcast only sets destination_device_identifyer address to special reserved address
// during the sending.
// So for broadcasting, the special reserved address value will be used as destination_device_identifyer,
// that special address is defined in transciever/config.rs as RESERVED_BROADCAST_IDENTIFYER.
// The duplication of that kind of message can be configured to be limited only by LifeTimeType
// value only, or by LifeTimeType value and additionally by voiding duplications within the network by the nodes.
transciever.broadcast("message", LifeTimeType(5), void_duplications=true);

// On sending the message to exact device, void_duplications parameter can be set.
// Setting of void_duplications to true - tells if the duplications are need to be voided
// by the nodes (receiving nodes or re-transmitting nodes).
transciever.send("message", DeviceIdentifyer(target_device_id), LifeTimeType(5), void_duplications=true);

// Sends the message to exact device (broadcast address is forbidden), and waits the `timeout`
// time for the message back from that device with acknowledge flag being set.
// In case, if the `acknowledge` message has been received successfully - Good result of calling of
// this methoid is returned, otherwise GuaranteeError is returned.
// The destination device may receive message one time, or multiple times. This can be configured
// by set void_duplications argument.
transciever.send_guaranteed("message", DeviceIdentifyer(target_device_id), LifeTimeType(5), timeout=SECOND * 2, void_duplications=true);
*/
