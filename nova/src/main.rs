#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]

//! # Nova
//!
//! The Nova kernel is a component of CharlotteOS, an experimental
//! modern operating system. The kernel is responsible for initializing the
//! hardware, providing commonizing abstractions for all hardware resources, and
//! managing the execution of user-space applications and the environment in
//! which they run. It is a crucial part of the operating system, as it provides
//! the foundation on which the rest of the system is built and it touches every
//! hardware and software component of the system on which it is used.

pub mod common;
pub mod console;
pub mod environment;
pub mod hal;
pub mod init;
pub mod log;
pub mod memory;

use core::panic::PanicInfo;

use embedded_graphics::mono_font::ascii::FONT_9X18;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use hal::drivers::display::limine_fb::FRAMEBUFFER;
use hal::isa::lp_control::interface::LpCtlIfce;
use hal::isa::lp_control::LpCtl;

/// This is the entry point for the kernel. The `main` function is called by the
/// bootloader after setting up the environment. It is made C ABI compatible so
/// that it can be called by Limine or any other Limine Boot Protocol compliant
/// bootloader.
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    logln!("Entering Nova.\nInitializing system...\n");
    let style = MonoTextStyle::new(&FONT_9X18, Rgb888::WHITE);
    Text::new("Entering Nova.\nInitializing system...\n", Point::new(0, 9), style)
        .draw(&mut *FRAMEBUFFER.lock())
        .unwrap();
    init::kernel_init();
    logln!("System initialized.\nHalting.\n");
    Text::new("System initialized.\nHalting.\n", Point::new(0, 8 + 36), style)
        .draw(&mut *FRAMEBUFFER.lock())
        .unwrap();
    LpCtl::halt()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    loop {}
}
