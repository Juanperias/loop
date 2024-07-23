use core::ptr;

pub fn memcpy(src: *mut u8, dest: *mut u8) {
    unsafe {
        ptr::write(src, ptr::read(dest));
    }
}
