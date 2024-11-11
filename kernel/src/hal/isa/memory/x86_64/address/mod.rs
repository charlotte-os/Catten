pub mod paddr;
pub mod vaddr;

use spin::Lazy;

use crate::hal::isa::memory::interface;

static PADDR_SIG_BITS: Lazy<u8> = Lazy::new(|| {
        let cpuid_val = unsafe { core::arch::x86_64::__cpuid(0x80000008) };
        let sig_bits = cpuid_val.eax & 0xff;
        sig_bits as u8
});

static PADDR_MASK: Lazy<usize> = Lazy::new(|| {
        let mask = (1 << *PADDR_SIG_BITS as usize) - 1;
        mask
});

static VADDR_SIG_BITS: Lazy<u8> = Lazy::new(|| {
        let cpuid_val = unsafe { core::arch::x86_64::__cpuid(0x80000008) };
        let sig_bits = cpuid_val.eax & (0xff << 8);
        sig_bits as u8
});

static VADDR_MASK: Lazy<usize> = Lazy::new(|| {
        let mask = (1 << *VADDR_SIG_BITS as usize) - 1;
        mask
});

pub struct x86_64_AddrTypes;

impl interface::AddrTypes for x86_64_AddrTypes {
        type PAddr = paddr::PAddr;
        type VAddr = vaddr::VAddr;
}
