use core::arch::asm;

pub fn disk_reset(drive: u8) {
    unsafe {
        asm!("mov ah, 0", "mov al, {}", "int 13h", in(reg_byte) drive);
    }
}
