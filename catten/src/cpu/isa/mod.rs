//! # Instruction Set Architecture (ISA) Interface
//!
//! This module provides a set of interfaces that commonize the ISA specific
//! functionality needed by the kernel.

mod common;
pub mod interface;
#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;
