#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::default_serial;
use mesh_lib::{init_node, AddressType, LifeTimeType, NodeConfig};
use panic_halt as _;

mod mesh_lib;

use mesh_lib::millis::ms;

use mesh_lib::NodeString;
use ufmt::uwrite;

use core::cell;

use mesh_lib::millis::traits::PlatformTime;

// Possible Values:
//
// ╔═══════════╦══════════════╦═══════════════════╗
// ║ PRESCALER ║ TIMER_COUNTS ║ Overflow Interval ║
// ╠═══════════╬══════════════╬═══════════════════╣
// ║        64 ║          250 ║              1 ms ║
// ║       256 ║          125 ║              2 ms ║
// ║       256 ║          250 ║              4 ms ║
// ║      1024 ║          125 ║              8 ms ║
// ║      1024 ║          250 ║             16 ms ║
// ╚═══════════╩══════════════╩═══════════════════╝
const PRESCALER: ms = 1024;
const TIMER_COUNTS: ms = 125;

const MILLIS_INCREMENT: ms = PRESCALER * TIMER_COUNTS / 16000;

static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<ms>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

struct Timer;

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    })
}

impl PlatformTime for Timer {
    type Timer = arduino_hal::pac::TC0;
    fn init(timer: Self::Timer) {
        timer.tccr0a.write(|w| w.wgm0().ctc());
        timer.ocr0a.write(|w| w.bits(TIMER_COUNTS as u8));
        timer.tccr0b.write(|w| match PRESCALER {
            8 => w.cs0().prescale_8(),
            64 => w.cs0().prescale_64(),
            256 => w.cs0().prescale_256(),
            1024 => w.cs0().prescale_1024(),
            _ => panic!(),
        });
        timer.timsk0.write(|w| w.ocie0a().set_bit());

        // Reset the global millisecond counter
        avr_device::interrupt::free(|cs| {
            MILLIS_COUNTER.borrow(cs).set(0);
        });

        unsafe { avr_device::interrupt::enable() };
    }
    fn millis() -> ms {
        avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    Timer::init(dp.TC0);

    let millis_fn_ptr = || Timer::millis();

    let mut mesh_node = init_node(NodeConfig {
        device_identifier: 1 as AddressType,
        listen_period: 150 as ms,
        usart: default_serial!(dp, pins, 9600),
        millis_fn_ptr,
    });

    let mut last_send_time: ms = { millis_fn_ptr }();
    let mut now_time: ms;
    let mut packet_counter: u32 = 0;

    loop {
        let _ = mesh_node.update();

        now_time = { millis_fn_ptr }();

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
