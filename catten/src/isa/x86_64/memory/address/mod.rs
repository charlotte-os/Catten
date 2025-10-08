pub mod paddr;
pub mod vaddr;

use lazy_static::lazy_static;

use crate::isa::interface::system_info::CpuInfoIfce;
use crate::isa::x86_64::system_info::CpuInfo;

lazy_static! {
    pub static ref PADDR_SIG_BITS: u8 = CpuInfo::get_paddr_sig_bits();
    pub static ref PADDR_MASK: usize = (1 << *PADDR_SIG_BITS as usize) - 1;
    pub static ref VADDR_SIG_BITS: u8 = CpuInfo::get_vaddr_sig_bits();
    pub static ref VADDR_MASK: usize = (1 << *VADDR_SIG_BITS as usize) - 1;
}
