use crate::memory::memread::memread;
use crate::memory::memset::memset;
use crate::modes::panic_mode::enter_panic_mode;

static mut MEMORY: *mut u8 = 0x2000 as *mut u8;
static mut OLD_MEMORY: *mut u8 = 0x2000 as *mut u8;

pub struct Memory {
    pub address: *mut u8,
    pub bytes: u8,
    pub freed: bool,
}

impl Memory {
    pub fn set(&self, pos: u8, byte: u8) {
        if pos <= self.bytes {
            memset(unsafe { self.address.offset(pos as isize) }, byte);
        } else {
            enter_panic_mode();
        }
    }
    pub fn get_offset(&self, pos: u8) -> *mut u8 {
        if pos <= self.bytes {
            unsafe { self.address.offset(pos as isize) }
        } else {
            enter_panic_mode();
            unsafe { self.address.offset(0) }
        }
    }
    pub fn get(&self, pos: u8) -> u8 {
        if pos <= self.bytes {
            memread(unsafe { self.address.offset(pos as isize) })
        } else {
            enter_panic_mode();
            0
        }
    }
    pub fn expand(&mut self, bytes_to_expand: u8) -> u8 {
        for i in 1..bytes_to_expand {
            memset(
                unsafe { self.address.offset(self.bytes as isize + i as isize) },
                0,
            );
        }
        self.bytes = self.bytes + bytes_to_expand;
        self.bytes
    }
}

pub fn free(mut address: Memory) {
    address.freed = true;
    unsafe {
        MEMORY = address.address;
        memset(address.get_offset(address.bytes + 1), 1);
    }
}

pub fn alloc(bytes: u8) -> Memory {
    let address = unsafe { MEMORY };

    for i in 0..bytes {
        memset(unsafe { address.offset(i as isize) }, 0);
    }

    if memread(unsafe { address.offset(bytes as isize + 1) }) == 1 {
        unsafe {
            memset(unsafe { address.offset(bytes as isize + 1) }, 0);

            unsafe {
                MEMORY = MEMORY.wrapping_add(1);
                OLD_MEMORY = OLD_MEMORY.wrapping_add(1);

                return Memory {
                    address,
                    bytes,
                    freed: false,
                };
            }
        }
    } else if memread(unsafe { address.offset(bytes as isize + 1) }) == 0 {
        unsafe {
            MEMORY = OLD_MEMORY;

            memset(unsafe { address.offset(bytes as isize + 1) }, 0);

            unsafe {
                MEMORY = MEMORY.wrapping_add(1);
                OLD_MEMORY = OLD_MEMORY.wrapping_add(1);

                return Memory {
                    address,
                    bytes,
                    freed: false,
                };
            }
        }
    } else {
        // 0 = freed: false,
        // 1 = freed: true,
        memset(unsafe { address.offset(bytes as isize + 1) }, 0);

        unsafe {
            MEMORY = MEMORY.wrapping_add(1);
            OLD_MEMORY = OLD_MEMORY.wrapping_add(1);

            return Memory {
                address,
                bytes,
                freed: false,
            };
        }
    }
}
