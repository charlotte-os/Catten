pub mod address;

use crate::llk::isa::current_isa::memory::address::paddr::PAddr;
use crate::llk::isa::current_isa::memory::address::vaddr::VAddr;
use crate::llk::isa::current_isa::memory::MemoryInterfaceImpl;
pub use crate::memory::vmem::{MemoryMapping, PageType};

pub trait MemoryInterface {
    type VAddr: address::VirtualAddress;
    type PAddr: address::PhysicalAddress;
    type Error;
    type AddressSpace: AddressSpaceInterface;
}

pub trait AddressSpaceInterface {
    fn get_current() -> Self;
    fn load(&self) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn find_free_region(
        &self,
        n_pages: usize,
        range: (VAddr, VAddr),
    ) -> Result<VAddr, <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn map_page(&mut self, mapping: MemoryMapping) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn unmap_page(&mut self, vaddr: VAddr) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn is_mapped(&self, vaddr: VAddr) -> Result<bool, <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn translate_address(&self, vaddr: VAddr) -> Result<PAddr, <MemoryInterfaceImpl as MemoryInterface>::Error>;
}
