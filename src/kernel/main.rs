#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod asm;
mod drivers;
mod fmt;
mod memory;
mod modes;
mod screen;
mod types;

use drivers::keyboard::{get_keyboard_pulse, KeyboardState};
use fmt::print::print_macros;

use memory::alloc::{alloc, free};
use memory::memread::memread;
use memory::memset::memset;
use modes::panic_mode::enter_panic_mode;
use screen::{
    clear::clear_screen,
    put::{new_line, putc, puts, Color},
};
use types::vec::Vec;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    puts("Welcome to loop kernel", Color::LightCyan);
    while true {
        let state = get_keyboard_pulse();
        if state[1] == KeyboardState::Pressed as u8 {
            putc(state[0], Color::LightCyan);
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
