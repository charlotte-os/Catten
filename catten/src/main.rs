#![no_std]
#![no_main]
#![feature(abi_custom)]
#![feature(allocator_api)]
#![feature(atomic_try_update)]
#![feature(exclusive_wrapper)]
#![feature(extend_one)]
#![feature(iter_advance_by)]
#![feature(likely_unlikely)]
#![feature(ptr_as_ref_unchecked)]
#![feature(slice_ptr_get)]
#![feature(step_trait)]
#![allow(static_mut_refs)]
#![allow(named_asm_labels)]

//! # Catten
//!
//! Catten is an operating system kernel developed as a component of CharlotteOS, an
//! experimental modern operating system.This kernel is responsible for initializing the hardware,
//! providing commonizing abstractions for all hardware resources, and managing the execution of
//! user-space applications and the environment in which they run. It is a crucial part of the
//! operating system, as it provides the foundation on which the rest of the system is built and it
//! touches every hardware and software component of the system on which it is used. While it is
//! developed as a component of CharlotteOS, it is designed to be modular and flexible, and thus
//! useful in other operating systems, embedded firmware, and other types of software distributions
//! as well.

extern crate alloc;

pub mod cabi;
pub mod common;
pub mod cpu;
pub mod drivers;
pub mod environment;
pub mod event;
pub mod framebuffer;
pub mod init;
pub mod log;
pub mod memory;
pub mod panic;
pub mod self_test;

use limine::mp::Cpu;
use spin::{Barrier, Lazy};

use crate::cpu::isa::interface::system_info::CpuInfoIfce;
use crate::cpu::isa::lp::ops::get_lp_id;
use crate::cpu::isa::system_info::CpuInfo;
#[cfg(target_arch = "x86_64")]
use crate::cpu::isa::timers::tsc::{IS_TSC_INVARIANT, TSC_CYCLE_PERIOD, TSC_FREQUENCY_HZ};
use crate::cpu::multiprocessor::get_lp_count;
use crate::cpu::multiprocessor::startup::{assign_id, start_secondary_lps};
use crate::cpu::scheduler::system_scheduler::SYSTEM_SCHEDULER;

const KERNEL_VERSION: (u64, u64, u64) = (0, 3, 5);
static INIT_BARRIER: Lazy<Barrier> = Lazy::new(|| Barrier::new(get_lp_count() as usize));
/// This is the bootstrap processor's entry point into the kernel. The `bsp_main` function is
/// called by the bootloader after setting up the environment. It is made C ABI compatible so
/// that it can be called by Limine or any other Limine Boot Protocol compliant bootloader.
#[unsafe(no_mangle)]
pub extern "C" fn bsp_main() -> ! {
    logln!(
        "Catten Kernel Version {}.{}.{}",
        (KERNEL_VERSION.0),
        (KERNEL_VERSION.1),
        (KERNEL_VERSION.2)
    );
    logln!("========================================================================");
    logln!("Initializing the system using the bootstrap processor...");
    unsafe {
        assign_id();
    }
    logln!("BSP assigned ID 0.");
    init::bsp_init();
    logln!("System initialized.");
    logln!("Starting secondary LPs...");
    start_secondary_lps().expect("Failed to start secondary LPs");
    INIT_BARRIER.wait();
    self_test::run_self_tests();
    #[cfg(target_arch = "x86_64")]
    {
        if *IS_TSC_INVARIANT {
            logln!("The x86-64 Timestamp Counter IS invariant.");
        } else {
            logln!("The x86-64 Timestamp Counter is NOT invariant.");
        }
        logln!(
            "The x86-64 Timestamp Counter frequency is {:?} MHz.",
            (*TSC_FREQUENCY_HZ / 1_000_000)
        );
        logln!(
            "The x86-64 Timestamp Counter period is {:?} picoseconds.",
            ((*TSC_CYCLE_PERIOD).as_picos())
        );
    }
    logln!("System Information:");
    logln!("CPU Vendor: {}", (CpuInfo::get_vendor()));
    logln!("CPU Model: {}", (CpuInfo::get_model()));
    logln!("Physical Address bits implemented: {}", (CpuInfo::get_paddr_sig_bits()));
    logln!("Virtual Address bits implemented: {}", (CpuInfo::get_vaddr_sig_bits()));
    logln!("LP{}: Bootstrapping complete. Yielding the processor to the scheduler.", (get_lp_id()));
    unsafe { SYSTEM_SCHEDULER.yield_lp() }
}
/// This is the application processors' entry point into the kernel. The `ap_main` function is
/// called by each application processor upon entering the kernel. It initializes the processor and
/// then hands it off to the scheduler. It is made C ABI compatible so that it can work with the
/// Limine Boot Protocol MP feature. Other boot protocols may require alternate implementations of
/// `ap_main`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ap_main(_cpuinfo: &Cpu) -> ! {
    unsafe {
        assign_id();
    }
    init::ap_init();
    INIT_BARRIER.wait();
    logln!("LP{}: Bootstrapping complete. Yielding the processor to the scheduler.", (get_lp_id()));
    unsafe { SYSTEM_SCHEDULER.yield_lp() }
}
