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

#[cfg(target_arch = "aarch64")]
mod aarch64;
// Contains components common to some though not necessarily all ISAs
mod common;
pub mod interface;
#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
