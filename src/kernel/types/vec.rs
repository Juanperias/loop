use crate::memory::alloc::Memory;
use crate::memory::alloc::{alloc, free};
use crate::memory::memread::memread;
use crate::memory::memset::memset;
use crate::modes::panic_mode::enter_panic_mode;
use crate::screen::put::{putc, Color};

pub struct Vec {
    pub pointer: u8,
    pub memory: Memory,
    pub bytes: u8,
}

impl Vec {
    pub fn new() -> Self {
        let vec_alloc = alloc(5);
        Vec {
            pointer: 0,
            memory: vec_alloc,
            bytes: 5,
        }
    }
    pub fn new_with_capacity(initial_bytes: u8) -> Self {
        let vec_alloc = alloc(initial_bytes);
        Vec {
            pointer: 0,
            memory: vec_alloc,
            bytes: initial_bytes,
        }
    }
    pub fn push(&mut self, value: u8) {
        if self.pointer == self.bytes {
            let new_bytes = self.memory.expand(5);
            self.bytes = new_bytes;
        }

        memset(self.memory.get_offset(self.pointer), value);

        self.pointer += 1;
    }
    pub fn get(&self, pos: u8) -> u8 {
        if pos < self.bytes {
            self.memory.get(pos)
        } else {
            enter_panic_mode();
            0
        }
    }
    pub fn pop(&mut self) {
        self.memory.set(self.pointer - 1, 0);
        self.pointer -= 1;
    }
    pub fn compare(&self, text: &str) -> bool {
        let bytes_slice = text.as_bytes();

        if bytes_slice.len() as u8 > self.pointer {
            return false;
        }

        if self.pointer > bytes_slice.len() as u8 {
            return false;
        }

        for (index, byte) in bytes_slice.iter().enumerate() {
            if &self.get(index as u8) != byte {
                return false;
            }
        }

        return true;
    }
    pub fn print(&self, color: Color) {
        for counter in 0..self.pointer {
            let element = self.get(counter);

            putc(element, color);
        }
    }
}
