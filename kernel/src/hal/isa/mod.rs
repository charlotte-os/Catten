//! # ISA
//! 
//! This module provides a set of interfaces that commonize the ISA specific functionalities
//! needed by the kernel. The interfaces are:
//! - [`Init`](init): provides the initialization and deinitialization functions.
//! - [`Memory`](memory): provides the memory related functions.
//! - [`Io`](io): provides the IO related functions.
//! - [`Instructions`](instructions): provides CPU instruction related functions.

pub mod init;
pub mod instructions;
pub mod io;
pub mod memory;
