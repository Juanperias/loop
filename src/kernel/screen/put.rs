use core::arch::asm;

pub fn putc(char: u8) {
    unsafe {
        asm!(
            "mov ah, 0x0e",
            "mov al, {}",
            "int 0x10",
            in(reg_byte) char
        )
    }
}

pub fn puts(text: &str) {
    putc(b' ');
    for letter in text.bytes() {
        putc(letter);
    }
}
