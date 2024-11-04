#![no_std]
#![no_main]

/// # Kernel Entry Point
/// 
/// This is the entry point for the kernel. The `main` function is called by the bootloader after
/// setting up the environment. The function is made C ABI compatible so that it can be called by
/// by Limine or any other Limine Boot Protocol compliant bootloader.

mod hal;

use core::panic::PanicInfo;

use hal::isa::instructions::{interface::InstructionInterface, InstructionWrapper};

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    InstructionWrapper::halt()
}

#[panic_handler]
fn rust_panic(_info: &PanicInfo) -> ! {
    loop {}
}
