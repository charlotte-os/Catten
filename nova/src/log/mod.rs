//! # Kernel Logging Macros
//!
//! This module provides convenient macros for logging messages to the kernel
//! log. They will be updated as the kernel develops to provide more
//! functionality and use an actual kernel log that will reside in memory and be
//! stored in a file. For now they print to the COM1 serial port on x86_64
//! systems only.

use core::borrow::BorrowMut;

//use embedded_term::*;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::hal::drivers::display::limine_fb::*;

/* lazy_static!{
    pub static ref GRAPHICAL_TERMINAL: Option<Mutex<Console<TextBufferCache<TextOnGraphic<Framebuffer>>>>> = {
        if let Some(fb) = Framebuffer::get() {
            Some(Mutex::new( Console::on_frame_buffer(fb) ))
        } else {
            None
        }
    };
} */

#[macro_export]
macro_rules! log {
    ($text:expr $(, $arg:tt)*) => ({
        use core::fmt::Write;
        if cfg! (target_arch = "x86_64") {
            use crate::hal::drivers::uart::uart_16550::LOG_PORT;
            let _ = write!(LOG_PORT.lock(), $text $(, $arg)*);
        }
/*         if let Some(terminal) = crate::log::GRAPHICAL_TERMINAL.as_ref() {
            let _ = write!(terminal.lock(), $text $(, $arg)*);
        } */
    })
}
#[macro_export]
macro_rules! logln {
    ($text:expr $(, $arg:tt)*) => ({
        use core::fmt::Write;
        if cfg! (target_arch = "x86_64") {
            use crate::hal::drivers::uart::uart_16550::LOG_PORT;
            let _ = writeln!(LOG_PORT.lock(), $text $(, $arg)*);
        }
        /* if let Some(terminal) = crate::log::GRAPHICAL_TERMINAL.as_ref() {
            let _ = writeln!(terminal.lock(), $text $(, $arg)*);
        } */
    })
}
