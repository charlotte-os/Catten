//! The x86-64 Instruction Set Architecture
//!
//! This module contains all x86-64-specific code. The two main reference documents
//! for this ISA are the
//! [AMD64 Architecture Programmer's Manual](https://docs.amd.com/v/u/en-US/40332-PUB_4.08)
//! which we refer to as the "AAPM"
//! and the
//! [Intel64 and IA-32 Architectures Software Developer's Manual](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
//! which we generally refer to as the "ISDM".

pub mod constants;
pub mod init;
pub mod interrupts;
pub mod io;
pub mod lp;
pub mod memory;
pub mod system_info;
pub mod timers;
