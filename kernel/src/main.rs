#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[allow(unused_imports)]
use vga_buffer::{print, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[allow(clippy::empty_loop)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::println!("Hello, World{}", '!');

    loop {}
}
