//! # TLB Management

use core::arch::asm;

use crate::isa::interface::system_info::CpuInfoIfce;
use crate::isa::x86_64::system_info::{CpuInfo, IsaExtension};
use crate::memory::vmem::{AddressSpaceId, VAddr};

pub fn invalidate_page(address_space: AddressSpaceId, vaddr: VAddr, page_count: u16) {
    todo!()
}

pub fn invalidate_global(vaddr: VAddr, page_count: u16) {
    let page_base = vaddr.page_offset();

    if CpuInfo::is_extension_supported(IsaExtension::Invlpgb) {
        /* On recent AMD processors we can use a shortcut in the form of the `invlpgb`
         * instruction. However we must ensure that it is supported prior to using it. It can
         * only be used to invalidate global kernel mappings because it doesn't account for
         * differing PCIDs between LPs */
        unsafe {
            asm!(
                // combine the address with the virtual address and global bits
                "or rax, 0b1001",
                // adjust this to be the number of additional pages to invalidate beyond one
                "sub ecx, 1",
                // the ASID and PCID register is cleared since we don't use those for global mappings
                "xor edx, edx",
                "invlpgb",
                // wait for all LPs to complete the shootdown
                "tlbsync",
                inout("rax") page_base => _,
                inout("ecx") page_count => _,
                out("edx") _
            );
        }
    } else {
        /* Use the x2APIC to raise a broadcast IPI to signal all logical processors to invalidate
         * their TLB entries for the specified address range in the specified address space.
         * This should make use of the charlottek IPI protocol and interrupt vector 32 (0x20) */
        todo!("main TLB shootdown sequence for global mappings")
    }
}
