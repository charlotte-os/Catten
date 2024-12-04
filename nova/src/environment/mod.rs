//! # Firmware Abstraction Layer
//!
//! This module provides an abstraction layer over the myriad firmware interfaces that are provided by modern hardware
//! platforms. It is intended to provide a common interface for interacting with device firmware and/or boot system
//! provided system description structures. Boot time firmware interactions are expected to be handled by
//! the bootloader and this module only provides a common interface over the supported boot protocols.

pub mod boot_protocol;
pub mod firmware;
