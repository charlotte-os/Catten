pub mod address;

use crate::cpu::isa::memory::Memory;
use crate::cpu::isa::memory::address::paddr::PAddr;
use crate::cpu::isa::memory::address::vaddr::VAddr;
pub use crate::memory::linear::{MemoryMapping, PageType};

pub trait MemoryIfce {
    type VAddr: address::VirtualAddress;
    type PAddr: address::PhysicalAddress;
    type Error;
    type HwAsid;
    type AddressSpace: AddressSpaceInterface;

    const HW_ASID_MAX: Self::HwAsid;
    const PAGE_SIZE: usize;
}

pub trait AddressSpaceInterface {
    fn get_current() -> Self;
    fn load(&self) -> Result<(), <Memory as MemoryIfce>::Error>;
    fn find_free_region(
        &mut self,
        n_pages: usize,
        range: (VAddr, VAddr),
    ) -> Result<VAddr, <Memory as MemoryIfce>::Error>;
    fn map_page(&mut self, mapping: MemoryMapping) -> Result<(), <Memory as MemoryIfce>::Error>;
    fn unmap_page(&mut self, vaddr: VAddr) -> Result<PAddr, <Memory as MemoryIfce>::Error>;
    fn is_mapped(&mut self, vaddr: VAddr) -> Result<bool, <Memory as MemoryIfce>::Error>;
    fn translate_address(&mut self, vaddr: VAddr) -> Result<PAddr, <Memory as MemoryIfce>::Error>;
}
