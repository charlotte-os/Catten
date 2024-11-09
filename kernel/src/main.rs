#![no_std]
#![no_main]

//! # The Lavender Kernel
//!
//! This operating system kernel is a component of Lavender, an experimental modern operating system.
//! The kernel is responsible for initializing the hardware, providing commonizing
//! abstractions for all hardware resources, and managing the execution of user-space
//! applications and the environment in which they run. It is a crucial part of the operating
//! system, as it provides the foundation on which the rest of the system is built and it
//! touches every hardware and software component of the system on which it is used.


mod hal;
mod log;

use core::panic::PanicInfo;

use hal::isa::init::{IsaInitializer, interface::InitInterface};
use hal::isa::instructions::{interface::InstructionInterface, InstructionWrapper};

/// This is the entry point for the kernel. The `main` function is called by the bootloader after
/// setting up the environment. The function is made C ABI compatible so that it can be called by
/// by Limine or any other Limine Boot Protocol compliant bootloader.
#[no_mangle]
unsafe extern "C" fn main() -> ! {
    logln!("Entering the Lavender Kernel...");
    logln!("Performing ISA specific initialization...");
    match IsaInitializer::init() {
        Ok(_) => logln!("ISA specific initialization complete."),
        Err(e) => {
            logln!("ISA specific initialization failed: {:?}", e)
        }
    }
    InstructionWrapper::halt()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    loop {}
}
