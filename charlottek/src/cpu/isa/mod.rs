//! # Instruction Set Architecture (ISA) Interface
//!
//! This module provides a set of interfaces that abstract the ISA specific
//! functionality needed by the kernel

pub mod interface;
#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(target_arch = "riscv64")]
pub use riscv64::*;
#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
