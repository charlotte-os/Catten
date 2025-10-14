pub mod paddr;
pub mod vaddr;

use spin::Lazy;

use crate::cpu::isa::interface::system_info::CpuInfoIfce;
use crate::cpu::isa::x86_64::system_info::CpuInfo;

pub static PADDR_SIG_BITS: Lazy<u8> = Lazy::new(CpuInfo::get_paddr_sig_bits);
pub static PADDR_MASK: Lazy<usize> = Lazy::new(|| (1 << *PADDR_SIG_BITS as usize) - 1);
pub static VADDR_SIG_BITS: Lazy<u8> = Lazy::new(CpuInfo::get_vaddr_sig_bits);
pub static VADDR_MASK: Lazy<usize> = Lazy::new(|| (1 << *VADDR_SIG_BITS as usize) - 1);
