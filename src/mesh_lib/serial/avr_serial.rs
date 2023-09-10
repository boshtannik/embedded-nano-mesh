use crate::mesh_lib::serial::GLOBAL_SERIAL;

use super::Usart;

pub fn init(serial: Usart) {
    avr_device::interrupt::free(|cs| {
        GLOBAL_SERIAL.borrow(cs).replace(Some(serial));
    })
}

#[macro_export]
macro_rules! serial_println {
    ($($arg:tt)*) => {
            ::avr_device::interrupt::free(|cs| {
            if let Some(serial) = &mut *crate::mesh_lib::serial::GLOBAL_SERIAL.borrow(cs).borrow_mut() {
                ::ufmt::uwriteln!(serial, $($arg)*).unwrap()  // TODO: Review this unwrap
            }
        })
    }
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        ::avr_device::interrupt::free(|cs| {
            if let Some(serial) = &mut *crate::mesh_lib::serial::GLOBAL_SERIAL.borrow(cs).borrow_mut() {
                ::ufmt::uwrite!(serial, $($arg)*)
            } else {
                Ok(())
            }
        })
    }
}

#[macro_export]
macro_rules! serial_write_byte {
    ($arg:expr) => {
        ::avr_device::interrupt::free(|cs| {
            if let Some(serial) = &mut *crate::mesh_lib::serial::GLOBAL_SERIAL
                .borrow(cs)
                .borrow_mut()
            {
                serial.write_byte($arg);
                Ok(()) as Result<(), ()>
            } else {
                Ok(())
            }
        })
    };
}

#[macro_export]
macro_rules! serial_try_read_byte {
    ($mutexed_celled_option_byte:expr) => {
        ::avr_device::interrupt::free(|cs| {
            if let Some(serial) = &mut *crate::mesh_lib::serial::GLOBAL_SERIAL
                .borrow(cs)
                .borrow_mut()
            {
                match serial.read() {
                    Ok(byte) => $mutexed_celled_option_byte.get_mut().replace(Some(byte)),
                    Err(_) => $mutexed_celled_option_byte.get_mut().replace(None),
                };
            } else {
                $mutexed_celled_option_byte.get_mut().replace(None);
            }
        })
    };
}

#[macro_export]
#[cfg(feature = "serial_debug")]
macro_rules! serial_debug {
    ($($arg:tt)*) => {
            ::avr_device::interrupt::free(|cs| {
            if let Some(serial) = &mut *crate::mesh_lib::serial::GLOBAL_SERIAL.borrow(cs).borrow_mut() {
                ::ufmt::uwriteln!(serial, $($arg)*).unwrap()  // TODO: Review this unwrap
            }
        })
    }
}

#[macro_export]
#[cfg(not(feature = "serial_debug"))]
macro_rules! serial_debug {
    ($($arg:tt)*) => {
        ()
    };
}
