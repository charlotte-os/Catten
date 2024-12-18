//! # Boot Protocol
//!
//! This module exists to abstract the boot protocol used to boot the kernel. As of now and for the foreseeable future
//! this kernel will only support the Limine boot protocol on top of UEFI or the subset of UEFI required by the EBBR
//! standard. This is in line with the system requirements for ARM64 systems including compliance with the requirements
//! of either the SystemReady or SystemReady DeviceTree bands of the ARM SystemReady program even if not formally
//! certified.
//!
//! Systems that do not meet the aforementioned requirements are not supported and will never be supported in line with
//! the philosophy of this project to exclusively support and actively encourage standardized systems. Please do not
//! open issues or PRs requesting or implementing support for non-standardized systems. They will be rejected. If you
//! would like support on a given device please contact the manufacturer and request that they enable support for
//! either the SystemReady or SystemReady DeviceTree band of the ARM SystemReady program.

pub mod limine;
