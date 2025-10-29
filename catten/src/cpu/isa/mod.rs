//! # Instruction Set Architecture (ISA) Interface
//!
//! This module provides a set of interfaces that commonize the ISA specific
//! functionality needed by the kernel:
//! - [`Initialization`](init): ISA specific system initialization and deinitialization
//! - [`Interrupts`](interrupts): wrappers over ISA specific interrupt management structures
//! - [`Input/Output`](io): wrappers over MMIO and Port IO
//! - [`Logical Processor Control`](lp): logical processor operating state control
//! - [`Memory`](memory): wrappers over ISA specific memory management structures
//! - [`System Information`](system_info): ISA specific system information

// Contains components common to some though not necessarily all ISAs
mod common;
pub mod interface;
#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "riscv64")]
pub use riscv64::*;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
