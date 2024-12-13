//! # Instruction Set Architecture (ISA) Interface
//!
//! This module provides a set of interfaces that commonize the ISA specific
//! functionality needed by the kernel:
//! - [`Initialization`](init): ISA specific system initialization and deinitialization
//! - [`Input/Output`](io): wrappers over MMIO and Port IO
//! - [`Logical Processor Control`](lp_control): logical processor operating state control
//! - [`Memory`](memory): wrappers over ISA specific memory management structures
//! - [`System Information`](system_info): ISA specific system information

pub mod init;
pub mod io;
pub mod lp_control;
pub mod memory;
pub mod system_info;
