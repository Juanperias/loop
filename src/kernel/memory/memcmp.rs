// 0 = equal
// 1 = address one != address two
use core::ptr;

pub fn memcmp(address_one: *mut u8, address_two: *mut u8) -> bool {
    let mut result = 0;

    let result = unsafe { ptr::read(address_one) == ptr::read(address_two) };
    result
}
