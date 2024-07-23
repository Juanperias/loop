use core::arch::asm;

pub fn x86_clear() {
    unsafe {
        asm!("xor ax, ax", "int 0x10", "mov ax, 0x003", "int 0x10");
    }
}
