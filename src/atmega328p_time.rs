use crate::mesh_lib::millis::ms;
use crate::mesh_lib::millis::traits::PlatformTime;
use core::cell;

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

pub static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<ms>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    })
}

pub fn init_timer(timer: arduino_hal::pac::TC0) {
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

pub struct Atmega328pTime;

impl PlatformTime for Atmega328pTime {
    fn millis() -> ms {
        avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get())
    }
}
