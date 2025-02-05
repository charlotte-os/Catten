pub mod address;
pub mod paging;

use crate::llk::isa::interface::memory::MemoryInterface;

pub enum Error {
    Unmapped,
    AlreadyMapped,
}
pub struct MemoryInterfaceImpl;

impl MemoryInterface for MemoryInterfaceImpl {
    type AddressSpace = paging::AddressSpace;
    type Error = Error;
    type PAddr = address::paddr::PAddr;
    type VAddr = address::vaddr::VAddr;
}
