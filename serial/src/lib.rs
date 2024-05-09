#![no_std]

use core::fmt::Write as _;

use spin_lock::once_lock::Lazy;
use uart_16550::SerialPort;

pub static SERIAL1: Lazy<SerialPort, fn() -> SerialPort> = Lazy::new(|| {
    let mut serial_port = unsafe { SerialPort::new(0x3f8) };
    serial_port.init();
    serial_port
});

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
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
