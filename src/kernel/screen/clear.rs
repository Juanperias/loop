use crate::screen::put::COUNTER;
use core::arch::asm;

const VGA: *mut u8 = 0xb8000 as *mut u8;

pub fn clear_screen() {
    unsafe {
        for i in COUNTER..0 {
            *VGA.offset(COUNTER as isize) = 0;
        }
        COUNTER = 0;
    }
}
