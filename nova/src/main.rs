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
pub mod framebuffer;
pub mod hal;
pub mod init;
pub mod log;
pub mod memory;

use core::panic::PanicInfo;

use hal::isa::interface::lp_control::LpCtlIfce;
use hal::isa::current_isa::lp_control::LpCtl;
use hal::isa::current_isa::system_info::*;
use hal::isa::interface::system_info::CpuInfoIfce;

/// This is the entry point for the kernel. The `main` function is called by the
/// bootloader after setting up the environment. It is made C ABI compatible so
/// that it can be called by Limine or any other Limine Boot Protocol compliant
/// bootloader.
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    logln!("Entering Nova.\nInitializing system...\n");
    init::kernel_init();
    logln!("System initialized.");
    logln!("System Information:");
    logln!("CPU Vendor: {:?}", (CpuInfo::get_vendor()));
    LpCtl::halt()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    loop {}
}
