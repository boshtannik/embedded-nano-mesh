use core::cell::Cell;

use crate::packet::DeviceAddress;
use arduino_hal::{clock::MHz16, hal::Atmega, pac::USART0, usart};

use self::sender::Transmitter;
use crate::packet::String64;

mod sender;

pub struct Transciever {
    my_address: DeviceAddress,
    transmitter: Transmitter,
    // receiver: Receiver,
}

enum Error {
    SomeError, // Dummy
}

type CelledSerialType = Cell<
    avr_hal_generic::usart::Usart<
        Atmega,
        USART0,
        avr_hal_generic::port::Pin<Input, PD0>,
        avr_hal_generic::port::Pin<Output, PD1>,
        MHz16,
    >,
>;

impl Transciever {
    pub fn new(my_address: DeviceAddress, reead_write: CelledSerialType) -> Transciever {
        Transciever {
            my_address,
            transmitter: Transmitter::new(),
        }
    }
    pub fn send_message(
        &mut self,
        message: String64,
        target_address: DeviceAddress,
    ) -> Result<(), Error> {
        // Split message into pieces, or not?

        Err(Error::SomeError)
    }
}
