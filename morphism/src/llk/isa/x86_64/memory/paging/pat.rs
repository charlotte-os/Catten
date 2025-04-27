//! # Page Attribute Table
//!
//! The Page Attribute Table (PAT) is a mechanism used in x86 processors to control the caching
//! behavior of memory pages. Morphism uses the PAT to set ordinary memory pages to write-back mode,
//! MMIO and DMA regions to strong uncacheable, and framebuffer pages to write combining. The CPU,
//! UEFI, and Limine all set the PAT to various states according to their standards however Morphism
//! modifies it at kernel boot to suit its own needs and does not alter it from that point forward.
//!
//! The PAT is a 64-bit register that is treated as an array of 8 8-bit entries, each holding a
//! value that represents the caching behavior of a memory page. The values set by Morphism are as
//! follows:
//!
//! | Value | Description |
//! |-------|-------------|
//! | 0     | Strong uncacheable |
//! | 1     | Write-combining |
//! | 2     | Writethrough |
//! | 3     | Strong uncacheable |
//! | 4     | Strong uncacheable |
//! | 5     | Write-combining |
//! | 6     | Writethrough |
//! | 7     | Strong uncacheable |
//!
//! This kernel assumes that target machines support the PAT and that the PAT is set to the values
//! specified above. x86-64 processors that do not support PAT are not supported by Morphism.

const PAT: u64 = 0x0001040000010400;

pub fn init_pat() {
    unsafe {
        core::arch::asm!("wrmsr IA32_PAT_MSR, {}", in(reg) PAT);
    }
}

pub enum CachingMode {
    StrongUncacheable = 0b00,
    WriteCombining = 0b01,
    WriteThrough = 0b10,
}
