//! # The Kernel Console
//!
//! The kernel console is a command processor that is used to interact with the kernel.
//! It is the primary interface for system administrators and kernel developers to interact with the kernel.
//! It is used to execute view logs, interact directly with the Nova kernel to override confiurations, display system
//! information, and more. The console is implemented as a module built into the kernel that can be conditionally
//! compiled in or out of the kernel or disabled at runtime. The kernel console can also be accessed through a serial
//! port as well as eventually USB CDC virtual serial ports for devices without typical input/output devices.

pub mod terminal;
