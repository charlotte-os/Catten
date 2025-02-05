//! # Instruction Set Architecture (ISA) Interface
//!
//! This module provides a set of interfaces that commonize the ISA specific
//! functionality needed by the kernel:
//! - [`Initialization`](init): ISA specific system initialization and deinitialization
//! - [`Input/Output`](io): wrappers over MMIO and Port IO
//! - [`Logical Processor Control`](lp_control): logical processor operating state control
//! - [`Memory`](memory): wrappers over ISA specific memory management structures
//! - [`System Information`](system_info): ISA specific system information

#[cfg(target_arch = "aarch64")]
pub mod aarch64;
pub mod interface;
#[cfg(target_arch = "aarch64")]
pub use aarch64 as current_isa;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64 as current_isa;
