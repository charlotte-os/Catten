//! # Firmware Abstraction Layer
//!
//! This module provides an abstraction layer over the myriad firmware interfaces that are provided
//! by modern hardware platforms. It is intended to provide a common interface for interacting with
//! device firmware and/or boot system provided system description structures. Boot time firmware
//! interactions are expected to be handled by the bootloader and this module only provides a common
//! interface over the supported boot protocols.
//!
//! - The Limine boot protocol is used on all supported systems. Accordingly all supported systems
//!   are required to provide at least a minimal EFI boot environment that can load a Limine boot
//!   protocol compatible bootloader such as Limine itself.
//! - PC like systems including all servers are expected to provide the UEFI and ACPI firmware
//!   interfaces in a manner that conforms to the specifications and does not require OS specific
//!   hacks.
//! - Embedded systems are expected to at least adhere to the Embedded Base Boot Requirements (EBBR)
//!   specification and provide a reduced subset of UEFI boot services and either ACPI tables or a
//!   Flattened Device Tree (FDT) if they do not provide full UEFI and ACPI conformant firmware.
//! - x86_64 systems tend to provide firmware operating in System Management Mode (SMM) however the
//!   interface to SMM interrupt calls is not standardized and thus must be accessed through ACPI.
//!   As such we do not provide a separate module for SMM calls.

// Advanced Configuration and Power Interface (ACPI)
mod acpi;
pub mod boot_protocol;
// Device Tree
#[cfg(not(target_arch = "x86_64"))]
mod devicetree;
// Unified Extensible Firmware Interface (UEFI) Runtime Services
mod uefi_rt;
