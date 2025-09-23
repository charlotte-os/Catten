pub mod address;

use crate::isa::memory::MemoryInterfaceImpl;
use crate::isa::memory::address::paddr::PAddr;
use crate::isa::memory::address::vaddr::VAddr;
pub use crate::memory::vmem::{MemoryMapping, PageType};

pub trait MemoryInterface {
    type VAddr: address::VirtualAddress;
    type PAddr: address::PhysicalAddress;
    type Error;
    type AddressSpace: AddressSpaceInterface;

    const PAGE_SIZE: usize;
}

pub trait AddressSpaceInterface {
    fn get_current() -> Self;
    fn load(&self) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn find_free_region(
        &mut self,
        n_pages: usize,
        range: (VAddr, VAddr),
    ) -> Result<VAddr, <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn map_page(
        &mut self,
        mapping: MemoryMapping,
    ) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn unmap_page(
        &mut self,
        vaddr: VAddr,
    ) -> Result<PAddr, <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn is_mapped(
        &mut self,
        vaddr: VAddr,
    ) -> Result<bool, <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn translate_address(
        &mut self,
        vaddr: VAddr,
    ) -> Result<PAddr, <MemoryInterfaceImpl as MemoryInterface>::Error>;
}
