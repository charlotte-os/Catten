//! # Logical Processor Control Interface for AArch64
use crate::cpu::isa::interface::lp;
use crate::cpu::isa::system_info::{CpuInfo, CpuInfoIfce};

pub type LpId = u32;

macro_rules! halt {
    () => {
        unsafe { core::arch::asm!("wfe", options(nomem, nostack, preserves_flags)) }
    };
}
macro_rules! mask_interrupts {
    () => {
        unsafe { core::arch::asm!("msr daifset, 0b1111", options(nomem, nostack)) }
    };
}
macro_rules! unmask_interrupts {
    () => {
        unsafe { core::arch::asm!("msr daifclr, 0b1111", options(nomem, nostack)) }
    };
}
