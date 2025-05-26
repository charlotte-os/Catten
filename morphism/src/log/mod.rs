//! # Kernel Logging Macros
//!
//! This module provides convenient macros for logging messages to the kernel
//! log. They will be updated as the kernel develops to provide more
//! functionality and use an actual kernel log that will reside in memory and be
//! stored in a file. For now they print to the COM1 serial port on x86_64
//! systems only.

#[macro_export]
macro_rules! log {
    ($text:expr $(, $arg:tt)*) => ({
         if cfg! (target_arch = "x86_64") {
            use core::fmt::Write;
            use crate::llk::drivers::uart::uart_16550::LOG_PORT;
            let _ = write!(LOG_PORT.lock(), $text $(, $arg)*);
        }
        use core::fmt::Write;
        use crate::log::print_prefix;

        print_prefix();
        crate::framebuffer::console::CONSOLE.lock().write_fmt(format_args!($($arg)*)).unwrap();
        crate::framebuffer::console::CONSOLE.lock().clear_inner_styling();
    })
}
#[macro_export]
macro_rules! logln {
    ($text:expr $(, $arg:tt)*) => ({
         if cfg! (target_arch = "x86_64") {
            use core::fmt::Write;
            use crate::llk::drivers::uart::uart_16550::LOG_PORT;
            let _ = writeln!(LOG_PORT.lock(), $text $(, $arg)*);
        }
        use core::fmt::Write;
        use crate::log::print_prefix;

        print_prefix();
        crate::framebuffer::console::CONSOLE.lock().write_fmt(format_args!($text $(, $arg)*)).unwrap();
        crate::framebuffer::console::CONSOLE.lock().write_char('\n', None, None);
        crate::framebuffer::console::CONSOLE.lock().clear_inner_styling();
    })
}
