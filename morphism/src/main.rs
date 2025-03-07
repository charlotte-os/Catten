#![no_std]
#![no_main]
#![feature(sync_unsafe_cell)]
#![feature(step_trait)]

//! # Purrnel
//!
//! The Purrnel kernel is a component of CharlotteOS, an experimental
//! modern operating system. The kernel is responsible for initializing the
//! hardware, providing commonizing abstractions for all hardware resources, and
//! managing the execution of user-space applications and the environment in
//! which they run. It is a crucial part of the operating system, as it provides
//! the foundation on which the rest of the system is built and it touches every
//! hardware and software component of the system on which it is used.

pub mod common;
pub mod framebuffer;
pub mod init;
pub mod llk;
pub mod log;
pub mod memory;
pub mod self_test;

use core::panic::PanicInfo;

use llk::isa::current_isa::lp_control::LpCtl;
use llk::isa::current_isa::system_info::CpuInfo;
use llk::isa::interface::lp_control::LpCtlIfce;
use llk::isa::interface::system_info::CpuInfoIfce;

/// This is the entry point for the kernel. The `main` function is called by the
/// bootloader after setting up the environment. It is made C ABI compatible so
/// that it can be called by Limine or any other Limine Boot Protocol compliant
/// bootloader.
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    logln!("Morphism Kernel Version 0.0.8");
    logln!("=========================");
    logln!("Initializing system...");
    init::kernel_init();
    logln!("System initialized.");
    logln!("System Information:");
    logln!("CPU Vendor: {:?}", (CpuInfo::get_vendor()));
    // TODO: Root cause the reason the following line halts execution without output or a panic.
    //logln!("CPU Model: {}", (CpuInfo::get_brand()));
    logln!("Physical Address bits implmented: {}", (CpuInfo::get_paddr_sig_bits()));
    logln!("Virtual Address bits implmented: {}", (CpuInfo::get_vaddr_sig_bits()));

    self_test::run_self_tests();

    logln!("Nothing left to do. Waiting for interrupts...");
    LpCtl::halt()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    logln!("{}", _info);
    LpCtl::halt()
}
