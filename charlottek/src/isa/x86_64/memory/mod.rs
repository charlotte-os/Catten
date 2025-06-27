pub mod address;
pub mod paging;

use crate::isa::current_isa::memory::address::paddr::PAddrError;
use crate::isa::interface::memory::MemoryInterface;
use crate::memory::pmem::Error as PMemError;
use crate::memory::vmem::Error as VMemError;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Unmapped,
    AlreadyMapped,
    NullVAddrNotAllowed,
    VAddrNotPageAligned,
    NoRequestedVAddrRegionAvailable,
    PMemError(PMemError),
    VMemError(VMemError),
}

impl From<PMemError> for Error {
    fn from(err: PMemError) -> Self {
        Error::PMemError(err)
    }
}

impl From<PAddrError> for Error {
    fn from(err: PAddrError) -> Self {
        Error::PMemError(PMemError::PAddrError(err))
    }
}

impl From<VMemError> for Error {
    fn from(err: VMemError) -> Self {
        Error::VMemError(err)
    }
}
pub struct MemoryInterfaceImpl;

impl MemoryInterface for MemoryInterfaceImpl {
    type AddressSpace = paging::AddressSpace;
    type Error = Error;
    type PAddr = address::paddr::PAddr;
    type VAddr = address::vaddr::VAddr;

    const PAGE_SIZE: usize = paging::PAGE_SIZE;
}
