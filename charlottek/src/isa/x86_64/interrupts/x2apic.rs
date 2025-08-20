//! # x2APIC Driver
//!
//! This module provides an interface for interacting with the x2APIC (Extended^2 Advanced
//! Programmable Interrupt Controller) in x86_64 systems. The x2APIC is an evolution of the original
//! xAPIC design, allowing for a larger number of processors and improved interrupt handling
//! capabilities as well as control through the use of MSRs instead of MMIO.
//! Note: Although it is referred to as a driver this module is in the ISA subsystem because local
//! interrupt controllers are very closely tied to the CPU ISA, in this case including being
//! accessed via MSR instructions. Consequently, rather than being placed in the top level device
//! driver module it makes more sense for it to reside here.

fn send_broadcast_ipis(lps: u32, vector: u8) {
    todo!("Implement broadcast IPI sending");
}
