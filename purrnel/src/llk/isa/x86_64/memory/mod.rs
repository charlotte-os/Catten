pub mod address;
pub mod paging;

use crate::llk::isa::interface::memory::MemoryInterface;
use crate::memory::pmem::Error as PMemError;

pub enum Error {
    Unmapped,
    AlreadyMapped,
    UnmappedTopLevelPageTable,
    PMemError(PMemError),
}

impl From<PMemError> for Error {
    fn from(err: PMemError) -> Self {
        Error::PMemError(err)
    }
}
pub struct MemoryInterfaceImpl;

impl MemoryInterface for MemoryInterfaceImpl {
    type AddressSpace = paging::AddressSpace;
    type Error = Error;
    type PAddr = address::paddr::PAddr;
    type VAddr = address::vaddr::VAddr;
}
