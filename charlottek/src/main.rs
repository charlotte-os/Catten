#![no_std]
#![no_main]
#![feature(allocator_api)]
#![feature(iter_advance_by)]
#![feature(likely_unlikely)]
#![feature(ptr_as_ref_unchecked)]
#![feature(slice_ptr_get)]
#![feature(step_trait)]
#![allow(static_mut_refs)]

//! # charlottek
//!
//! charlottek is an operating system kernel developed as a component of CharlotteOS, an
//! experimental modern operating system.This kernel is responsible for initializing the hardware,
//! providing commonizing abstractions for all hardware resources, and managing the execution of
//! user-space applications and the environment in which they run. It is a crucial part of the
//! operating system, as it provides the foundation on which the rest of the system is built and it
//! touches every hardware and software component of the system on which it is used. While it is
//! developed as a component of CharlotteOS, it is designed to be modular and flexible, and thus
//! useful in other operating systems, embedded firmware, and other types of software distributions
//! as well.

extern crate alloc;

pub mod drivers;
pub mod environment;
pub mod framebuffer;
pub mod init;
pub mod isa;
pub mod klib;
pub mod log;
pub mod memory;
pub mod panic;
pub mod self_test;
pub mod tsmp;

use isa::current_isa::lp_control::LpControl;
use isa::current_isa::system_info::CpuInfo;
use isa::interface::lp_control::LpControlIfce;
use isa::interface::system_info::CpuInfoIfce;
use limine::mp::Cpu;

/// This is the bootstrap processor's entry point into the kernel. The `bsp_main` function is
/// called by the bootloader after setting up the environment. It is made C ABI compatible so
/// that it can be called by Limine or any other Limine Boot Protocol compliant bootloader.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bsp_main() -> ! {
    logln!("charlottek Kernel Version 0.1.0");
    logln!("=========================");
    logln!("Initializing system...");
    init::kernel_init();
    logln!("System initialized.");
    logln!("System Information:");
    logln!("CPU Vendor: {:?}", (CpuInfo::get_vendor()));
    // TODO: Root cause the reason the following line halts execution without output or a panic.
    //logln!("CPU Model: {}", (CpuInfo::get_brand()));
    logln!(
        "Physical Address bits implmented: {}",
        (CpuInfo::get_paddr_sig_bits())
    );
    logln!(
        "Virtual Address bits implmented: {}",
        (CpuInfo::get_vaddr_sig_bits())
    );

    logln!("Starting secondary LPs...");
    tsmp::mp::start_secondary_lps().expect("Failed to start secondary LPs");

    self_test::run_self_tests();

    logln!("Nothing left to do. Waiting for interrupts...");
    LpControl::halt()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ap_main(_cpuinfo: &Cpu) -> ! {
    let lp_identifier = LpControl::get_lp_id();
    logln!(
        "Logical Processor with x2APIC ID = {} has entered charlottek via ap_main",
        lp_identifier
    );
    LpControl::halt()
}
