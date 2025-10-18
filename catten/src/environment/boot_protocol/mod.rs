//! # Boot Protocol
//!
//! This module exists to abstract the boot protocol used to boot the kernel. As of now and for the
//! foreseeable future this kernel will only support the Limine boot protocol on top of UEFI or the
//! subset of UEFI required by the EBBR standard.
//!
//! Systems that do not meet the aforementioned requirements are not supported and will never be
//! supported in line with the philosophy of this project to exclusively support and actively
//! encourage the proliferation of standardized systems. Please do not open issues or PRs requesting
//! or implementing support for non-standardized systems. They will be rejected.

pub mod limine;
