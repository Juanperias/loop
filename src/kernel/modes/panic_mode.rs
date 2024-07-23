use core::arch::asm;

extern "C" {
    fn panic_mode();
}

pub fn enter_panic_mode() {
    unsafe {
        panic_mode();
    }
}
