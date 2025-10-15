//! Aarch64 Instruction Set Architecture
//!
//! This module contains all aarch64-specific code. The main reference documentation
//! for this ISA is the
//! [ARM Architecture Reference Manual, for A-profile architecture](https://developer.arm.com/documentation/ddi0487/latest/)
//! which we generally refer to as the "ARM ARM".

pub mod init;
pub mod interrupts;
pub mod io;
pub mod lp;
pub mod memory;
pub mod system_info;
