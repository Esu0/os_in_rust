#![no_std]

use core::{fmt::Write as _, ops::{Deref, DerefMut}};

use spin_lock::once_lock::OnceMutex;
use uart_16550::SerialPort;

pub struct Serial1(SerialPort);

impl Default for Serial1 {
    fn default() -> Self {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Serial1(serial_port)
    }
}

impl Deref for Serial1 {
    type Target = SerialPort;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Serial1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub static SERIAL1: OnceMutex<Serial1> = OnceMutex::new();

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {$crate::_print(format_args!($($arg)*))};
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
