#![no_std]
#![no_main]
#![feature(abi_custom)]
#![feature(allocator_api)]
#![feature(atomic_try_update)]
#![feature(iter_advance_by)]
#![feature(likely_unlikely)]
#![feature(ptr_as_ref_unchecked)]
#![feature(slice_ptr_get)]
#![feature(step_trait)]
#![allow(static_mut_refs)]
#![allow(named_asm_labels)]

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
pub mod event;
pub mod framebuffer;
pub mod init;
pub mod isa;
pub mod klib;
pub mod log;
pub mod memory;
pub mod multiprocessing;
pub mod panic;
pub mod scheduler;
pub mod self_test;

use isa::interface::system_info::CpuInfoIfce;
use isa::target::lp;
use isa::target::system_info::CpuInfo;
use limine::mp::Cpu;

/// This is the bootstrap processor's entry point into the kernel. The `bsp_main` function is
/// called by the bootloader after setting up the environment. It is made C ABI compatible so
/// that it can be called by Limine or any other Limine Boot Protocol compliant bootloader.
#[unsafe(no_mangle)]
pub extern "C" fn bsp_main() -> ! {
    logln!("charlottek Kernel Version 0.1.0");
    logln!("=========================");
    logln!("Initializing the system using the bootstrap processor...");
    init::bsp_init();
    logln!("System initialized.");
    logln!("Starting secondary LPs...");
    multiprocessing::start_secondary_lps().expect("Failed to start secondary LPs");
    self_test::run_self_tests();
    logln!("System Information:");
    logln!("CPU Vendor: {}", (CpuInfo::get_vendor()));
    logln!("CPU Model: {}", (CpuInfo::get_model()));
    logln!("Physical Address bits implmented: {}", (CpuInfo::get_paddr_sig_bits()));
    logln!("Virtual Address bits implmented: {}", (CpuInfo::get_vaddr_sig_bits()));
    logln!("Nothing left to do. Waiting for interrupts...");
    lp::halt!()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ap_main(_cpuinfo: &Cpu) -> ! {
    let mut id_ctr_lock = multiprocessing::id_counter.lock();
    let lp_id = *id_ctr_lock;
    *id_ctr_lock += 1;
    drop(id_ctr_lock);
    lp::store_lp_id(lp_id);
    logln!(
        "Logical Processor {} with local interrupt controller ID = {} has entered charlottek via ap_main",
        (lp::get_lp_id()),
        (lp::get_lic_id!())
    );
    lp::halt!()
}
