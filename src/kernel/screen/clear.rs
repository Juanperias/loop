use crate::screen::put::COUNTER;
use core::arch::asm;

const VGA: *mut u8 = 0xb8000 as *mut u8;

pub fn clear_one_char() {
    unsafe {
        *VGA.offset((COUNTER * 2).into()) = 0;
        *VGA.offset((COUNTER * 2 + 1).into()) = 0;
        if COUNTER > 0 {
            COUNTER -= 1
        }
    }
}

pub fn clear_screen() {
    unsafe {
        for i in COUNTER..0 {
            *VGA.offset(COUNTER as isize) = 0;
        }
        COUNTER = 0;
    }
}
