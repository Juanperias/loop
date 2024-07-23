use core::ptr;

pub fn memread(address: *mut u8) -> u8 {
    let value = unsafe { ptr::read(address) };

    value
}
