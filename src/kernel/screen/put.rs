pub static mut COUNTER: u8 = 0;
const VGA: *mut u8 = 0xb8000 as *mut u8;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub fn putc(char: u8, color: Color) {
    unsafe {
        *VGA.offset(COUNTER as isize * 2) = char;
        *VGA.offset(COUNTER as isize * 2 + 1) = color as u8;

        COUNTER += 1;
    }
}

pub fn new_line() {
    // Calcula la posición actual en el buffer VGA
    unsafe {
        let current_position = (COUNTER as usize) % (BUFFER_WIDTH * BUFFER_HEIGHT);

        // Calcula la posición del siguiente renglón
        let next_row_position = ((current_position / BUFFER_WIDTH) + 1) * BUFFER_WIDTH;

        // Llena el espacio entre la posición actual y la siguiente fila con espacios en blanco
        for i in current_position..next_row_position {
            putc(b' ', Color::Black);
        }

        // Actualiza el contador para la siguiente línea
        COUNTER = next_row_position as u8;
    }
}

pub fn puts(text: &str, color: Color) {
    for byte in text.bytes() {
        putc(byte, color);
    }

    new_line();
}
