pub mod address;

use crate::memory::vmem::MemoryMapping;

pub trait MemoryInterface {
    type VAddr: address::VirtualAddress;
    type PAddr: address::PhysicalAddress;
/*     type Error;

    fn find_free_region(addr_space: usize, n_pages: usize) -> Result<Self::VAddr, Self::Error>;
    fn map_page(addr_space: usize, mapping: MemoryMapping) -> Result<(), Self::Error>;
    fn unmap_page(addr_space: usize, vaddr: Self::VAddr) -> Result<MemoryMapping, Self::Error>;
    fn is_mapped(addr_space: usize, vaddr: Self::VAddr) -> Result<bool, Self::Error>;
    fn get_mapping(addr_space: usize, vaddr: Self::VAddr) -> Result<MemoryMapping, Self::Error>; */
}


