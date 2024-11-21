#![no_std]
#![no_main]

//! # The Lavender Kernel
//!
//! This operating system kernel is a component of Lavender, an experimental
//! modern operating system. The kernel is responsible for initializing the
//! hardware, providing commonizing abstractions for all hardware resources, and
//! managing the execution of user-space applications and the environment in
//! which they run. It is a crucial part of the operating system, as it provides
//! the foundation on which the rest of the system is built and it touches every
//! hardware and software component of the system on which it is used.

mod environment;
mod hal;
mod init;
mod log;
mod memory;

use core::panic::PanicInfo;

use hal::isa::lp_control::interface::LpCtlIfce;
use hal::isa::lp_control::LpCtl;

/// This is the entry point for the kernel. The `main` function is called by the
/// bootloader after setting up the environment. It is made C ABI compatible so
/// that it can be called by Limine or any other Limine Boot Protocol compliant
/// bootloader.
#[no_mangle]
unsafe extern "C" fn main() -> ! {
        logln!("Entering the Lavender Kernel...");
        init::kernel_init();
        LpCtl::halt()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
        logln!("{}", _info);
        loop {}
}
