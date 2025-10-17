//! # Firmware Interface Modules
//!
//! This module contains the firmware interface modules that provide access to the firmware-provided
//! services and data structures. These modules are used to interact with the firmware to obtain
//! information about the hardware and the environment in which the kernel is running and perform
//! power management. The firmware interface modules are optional and can be enabled or disabled
//! using feature flags to suit the requirements of the target platform.
//!
//! - PC like systems including servers are expected to provide the UEFI and ACPI firmware
//!   interfaces in a manner that complies with the specifications and does not require OS specific
//!   hacks.
//! - Embedded systems are expected to adhere to the Embedded Base Boot Requirements (EBBR)
//!   specification and provide a reduced subset of UEFI boot services and either ACPI tables or a
//!   Flattened Device Tree (FDT).
//! - All ARM64 systems are expected to provide an ARM Trusted Firmware (ATF) and consequently a
//!   Secure Monitor Call (SMC) interface.
//! - x86_64 systems tend to provide firmware operating in System Management Mode (SMM) however the
//!   interface to SMM interrupt calls is not standardized and thus must be accessed through ACPI.
//!   As such we do not provide a separate module for SMM calls.

// Advanced Configuration and Power Interface (ACPI)
mod acpi;
// Device Tree
#[cfg(not(target_arch = "x86_64"))]
mod devicetree;
// ARM Secure Monitor Call (SMC) Interface
#[cfg(target_arch = "aarch64")]
mod arm_smc;
// Unified Extensible Firmware Interface (UEFI) Runtime Services
mod uefi_rt;
