pub mod paddr;
pub mod vaddr;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref PADDR_SIG_BITS: u8 = {
        let cpuid_val = unsafe { core::arch::x86_64::__cpuid(0x80000008) };
        let sig_bits = cpuid_val.eax & 0xff;
        sig_bits as u8
    };
    pub static ref PADDR_MASK: usize = (1 << *PADDR_SIG_BITS as usize) - 1;
    pub static ref VADDR_SIG_BITS: u8 = {
        let cpuid_val = unsafe { core::arch::x86_64::__cpuid(0x80000008) };
        let sig_bits = (cpuid_val.eax & (0xff << 8)) >> 8;
        sig_bits as u8
    };
    pub static ref VADDR_MASK: usize = (1 << *VADDR_SIG_BITS as usize) - 1;
}
