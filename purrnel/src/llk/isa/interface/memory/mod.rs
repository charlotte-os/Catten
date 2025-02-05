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
    fn find_free_region(&self, n_pages: usize) -> Result<VAddr, <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn map_page(&mut self, mapping: MemoryMapping) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn unmap_page(&mut self, vaddr: VAddr) -> Result<MemoryMapping, <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn is_mapped(&self, vaddr: VAddr) -> Result<bool, <MemoryInterfaceImpl as MemoryInterface>::Error>;
    fn get_mapping(&self, vaddr: VAddr) -> Result<MemoryMapping, <MemoryInterfaceImpl as MemoryInterface>::Error>;
}
