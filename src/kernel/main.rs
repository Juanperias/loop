#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod disk;
mod fmt;
mod memory;
mod modes;
mod screen;

mod types;

use disk::reset::disk_reset;
use fmt::print::print_macros;

use memory::alloc::{alloc, free};
use memory::memread::memread;
use memory::memset::memset;
use modes::panic_mode::enter_panic_mode;
use screen::{
    clear::x86_clear,
    put::{putc, puts},
};
//use types::vec::Vec;

#[no_mangle]
pub extern "C" fn main() -> ! {
    x86_clear();
    disk_reset(0);

    puts("No ha petado");

    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
