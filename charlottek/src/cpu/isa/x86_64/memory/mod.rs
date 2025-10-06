pub mod address;
pub mod paging;
pub mod tlb;

pub use crate::cpu::isa::interface::memory::MemoryIfce;
use crate::cpu::isa::memory::address::paddr::PAddrError;
use crate::memory::linear::Error as VMemError;
use crate::memory::physical::Error as PMemError;

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

pub struct Memory;

impl MemoryIfce for Memory {
    type AddressSpace = paging::AddressSpace;
    type Error = Error;
    type HwAsid = u16;
    type PAddr = address::paddr::PAddr;
    type VAddr = address::vaddr::VAddr;

    const HW_ASID_MAX: Self::HwAsid = (1 << 12) - 1;
    const PAGE_SIZE: usize = paging::PAGE_SIZE;
}
