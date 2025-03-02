//! # Firmware Interface Modules
//!
//! This module contains the firmware interface modules that provide access to the firmware-provided
//! services and data structures. These modules are used to interact with the firmware to obtain
//! information about the hardware and the environment in which the kernel is running and perform power management.
//! The firmware interface modules are optional and can be enabled or disabled using feature flags to suit the
//! requirements of the target platform.
//!
//! *PC like systems arr expected to provide UEFI, ACPI, and SMBIOS firmware interfaces.
//! *Embedded systems are expected to adhere to the Embedded Base Boot Requirements (EBBR) specification and provide
//! a reduced subset of UEFI boot services and either ACPI tables or a Flattened Device Tree (FDT).
//! *All ARM64 systems are expected to provide an ARM Trusted Firmware (ATF) and consequently a Secure Monitor Call
//! (SMC) interface. *All RISC-V systems are expected to provide firmware running in M-mode that implements the
//! Supervisor Binary Interface (SBI). *x86_64 systems do all tend to provide firmware operating in System Management
//! Mode (SMM) however the interface to SMM interrupt calls is not standardized and thus must be accessed through the
//! use of ACPI AML code. As such we do not provide a separate module for SMM calls.

// Advanced Configuration and Power Interface (ACPI)
pub mod acpi;
// Flattened Device Tree (FDT) aka Device Tree Blob (DTB)
#[cfg(not(target_arch = "x86_64"))]
pub mod fdt;
// RISC-V Supervisor Binary Interface (SBI)
#[cfg(target_arch = "riscv64")]
pub mod sbi;
// System Management BIOS (SMBIOS)
pub mod smbios;
// ARM Secure Monitor Calls (SMC)
#[cfg(target_arch = "aarch64")]
pub mod smc;
// Unified Extensible Firmware Interface (UEFI) Runtime Services
pub mod uefi_rt;
