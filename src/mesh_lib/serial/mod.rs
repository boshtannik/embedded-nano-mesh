use avr_device::interrupt::Mutex;
use core::cell::RefCell;

mod avr_serial;

pub type Usart = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
pub static GLOBAL_SERIAL: Mutex<RefCell<Option<Usart>>> = Mutex::new(RefCell::new(None));

pub use avr_serial::init;
