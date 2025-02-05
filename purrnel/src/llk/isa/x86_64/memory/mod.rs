pub mod address;
pub mod paging;

pub struct MemoryInterfaceImpl;

impl MemoryInterface for MemoryInterfaceImpl {
    type AddressSpace = AddressSpace;
    type Error = Error;
    type PAddr = address::paddr::PAddr;
    type VAddr = address::vaddr::VAddr;
}

pub enum Error {
    Unmapped,
    AlreadyMapped,
}