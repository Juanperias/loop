#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod fmt;
mod memory;
mod modes;
mod screen;

mod types;

use fmt::print::print_macros;

use memory::alloc::{alloc, free};
use memory::memread::memread;
use memory::memset::memset;
use modes::panic_mode::enter_panic_mode;
use screen::{
    clear::clear_screen,
    put::{putc, puts, Color},
};
//use types::vec::Vec;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
