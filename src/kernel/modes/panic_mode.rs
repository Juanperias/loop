use crate::screen::{clear::clear_screen, put::puts, put::Color};

pub fn enter_panic_mode() -> ! {
    clear_screen();

    puts("PANIC IN THE SYSTEM", Color::Red);

    loop {}
}
