use spin::lazy::Lazy;

// Re-export the common 64-bit address modules for AArch64
/* Note: The x86-64 canonical address rules should work on aarch64 systems if they are
 * configured correctly. Let's endeavor to do this to every extent possible for portability. */
use crate::cpu::isa::aarch64::system_info::CpuInfo;
pub use crate::cpu::isa::common::memory::address::*;
use crate::cpu::isa::interface::system_info::CpuInfoIfce;

pub static PADDR_MASK: Lazy<usize> =
    Lazy::new(|| (1 << CpuInfo::get_paddr_sig_bits() as usize) - 1);
pub static VADDR_MASK: Lazy<usize> =
    Lazy::new(|| (1 << CpuInfo::get_vaddr_sig_bits() as usize) - 1);
