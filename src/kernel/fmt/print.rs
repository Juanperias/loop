use crate::screen::put::{putc, puts};

pub mod print_macros {
    #[macro_export]
    macro_rules! println {
        ($text: expr, $($args:tt)*) => {
            for byte in $text.bytes() {
                if byte == b'%' {
                    putc($($args)*);
                } else {
                    putc(byte);
                }
            }
        };
    }
}
