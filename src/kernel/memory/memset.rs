use core::ptr;

pub fn memset(src: *mut u8, value: u8) {
    unsafe { ptr::write(src, value) };
}
