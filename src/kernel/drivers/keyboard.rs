use crate::asm::inb::inb;
use crate::asm::outb::outb;
use crate::screen::clear::clear_one_char;
use crate::screen::put::{new_line, putc, Color};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyboardState {
    Pressed = 0,
    Released = 1,
    Nothing = 2,
    Event = 3,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KeyboardEvents {
    Backspace = 8,
    Enter = 13,
}

pub fn get_char_from_scan_code(scan_code: u8) -> [u8; 2] {
    let result = match scan_code {
        0x1E => [b'a', KeyboardState::Pressed as u8], // a is pressed
        0x9E => [b'a', KeyboardState::Released as u8], // a is released

        0x30 => [b'b', KeyboardState::Pressed as u8], // b is pressed
        0xB0 => [b'b', KeyboardState::Released as u8], // b is released

        0x2E => [b'c', KeyboardState::Pressed as u8], // c is pressed
        0xAE => [b'c', KeyboardState::Released as u8], // c is released

        0x20 => [b'd', KeyboardState::Pressed as u8],
        0xA0 => [b'd', KeyboardState::Released as u8],

        0x12 => [b'e', KeyboardState::Pressed as u8],
        0x92 => [b'e', KeyboardState::Released as u8],

        0x21 => [b'f', KeyboardState::Pressed as u8],
        0xA1 => [b'f', KeyboardState::Released as u8],

        0x22 => [b'g', KeyboardState::Pressed as u8],
        0xA2 => [b'g', KeyboardState::Released as u8],

        0x23 => [b'h', KeyboardState::Pressed as u8],
        0xA3 => [b'h', KeyboardState::Released as u8],

        0x17 => [b'i', KeyboardState::Pressed as u8],
        0x97 => [b'i', KeyboardState::Released as u8],

        0x24 => [b'j', KeyboardState::Pressed as u8],
        0xA4 => [b'j', KeyboardState::Released as u8],

        0x25 => [b'k', KeyboardState::Pressed as u8],
        0xA5 => [b'k', KeyboardState::Released as u8],

        0x26 => [b'l', KeyboardState::Pressed as u8],
        0xA6 => [b'l', KeyboardState::Released as u8],

        0x32 => [b'm', KeyboardState::Pressed as u8],
        0x3A => [b'm', KeyboardState::Released as u8],

        0x31 => [b'n', KeyboardState::Pressed as u8],
        0xB1 => [b'n', KeyboardState::Released as u8],

        0x18 => [b'o', KeyboardState::Pressed as u8],
        0x98 => [b'o', KeyboardState::Released as u8],

        0x19 => [b'p', KeyboardState::Pressed as u8],
        0x99 => [b'p', KeyboardState::Released as u8],

        0x10 => [b'q', KeyboardState::Pressed as u8],
        0x90 => [b'q', KeyboardState::Released as u8],

        0x13 => [b'r', KeyboardState::Pressed as u8],
        0x93 => [b'r', KeyboardState::Released as u8],

        0x1F => [b's', KeyboardState::Pressed as u8],
        0x9F => [b's', KeyboardState::Released as u8],

        0x14 => [b't', KeyboardState::Pressed as u8],
        0x94 => [b't', KeyboardState::Released as u8],

        0x16 => [b'u', KeyboardState::Pressed as u8],
        0x96 => [b'u', KeyboardState::Released as u8],

        0x2F => [b'v', KeyboardState::Pressed as u8],
        0xAF => [b'v', KeyboardState::Released as u8],

        0x11 => [b'w', KeyboardState::Pressed as u8],
        0x91 => [b'w', KeyboardState::Released as u8],

        0x2D => [b'x', KeyboardState::Pressed as u8],
        0xAD => [b'x', KeyboardState::Released as u8],

        0x15 => [b'y', KeyboardState::Pressed as u8],
        0x95 => [b'y', KeyboardState::Released as u8],

        0x02 => [b'1', KeyboardState::Pressed as u8], // 1 is pressed
        0x82 => [b'1', KeyboardState::Released as u8], // 1 is released

        0x2C => [b'z', KeyboardState::Pressed as u8],
        0xAC => [b'z', KeyboardState::Released as u8],

        0x03 => [b'2', KeyboardState::Pressed as u8],
        0x83 => [b'2', KeyboardState::Released as u8],

        0x04 => [b'3', KeyboardState::Pressed as u8],
        0x84 => [b'3', KeyboardState::Released as u8],

        0x05 => [b'4', KeyboardState::Pressed as u8],
        0x85 => [b'4', KeyboardState::Released as u8],

        0x06 => [b'5', KeyboardState::Pressed as u8],
        0x86 => [b'5', KeyboardState::Released as u8],

        0x07 => [b'6', KeyboardState::Pressed as u8],
        0x87 => [b'6', KeyboardState::Released as u8],

        0x08 => [b'7', KeyboardState::Pressed as u8],
        0x88 => [b'7', KeyboardState::Released as u8],

        0x09 => [b'8', KeyboardState::Pressed as u8],
        0x89 => [b'8', KeyboardState::Released as u8],

        0x0A => [b'9', KeyboardState::Pressed as u8],
        0x8A => [b'9', KeyboardState::Released as u8],

        0x0B => [b'0', KeyboardState::Pressed as u8],
        0x8B => [b'0', KeyboardState::Released as u8],

        0x39 => [b' ', KeyboardState::Pressed as u8],
        0xB9 => [b' ', KeyboardState::Released as u8],

        0x0E => [8, KeyboardState::Event as u8],
        0x1C => [13, KeyboardState::Event as u8],

        _ => [b'?', KeyboardState::Nothing as u8],
    };

    result
}

pub fn get_keyboard_pulse() -> [u8; 2] {
    while (inb(0x64) & 0x01) == 0 {}

    let scan_code = inb(0x60);

    let result = get_char_from_scan_code(scan_code);

    if result[1] == KeyboardState::Event as u8 {
        if result[0] == KeyboardEvents::Enter as u8 {
            new_line();
        } else if result[0] == KeyboardEvents::Backspace as u8 {
            clear_one_char();
        }
    }

    result
}
