//! # Hardware Abstraction Layer
//!
//! This module contains the hardware abstraction layer of the kernel. It is responsible for providing an interface to
//! system hardware for the rest of the kernel. It deals with the specifics of the hardware configuration and operation
//! and provides a common set of interfaces for the rest of the kernel to use.

pub mod bootinfo;
pub mod drivers;
pub mod isa;
