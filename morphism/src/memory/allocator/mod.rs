//! Double Free List Allocator
//!
//! This module provides a kernel dynamic memory allocator implementation that uses two free list to
//! track free regions of memory. One list is sorted by address and the other by size. Allocations
//! may be of any size, however it is recommended to not use it for frequent small allocations as
//! the allocator metadata will use far more memory than the actual allocations. However since most
//! kernel allocations are expected to be anywhere from medium to gargantuan in size, this is not
//! expected to be an issue.

use super::pmem::{Error as PMemError, VAddr, PHYSICAL_FRAME_ALLOCATOR};
use super::vmem::{MemoryMapping, PageType};
use crate::common::raw_ds::raw_vec::RawVec;
use crate::llk::isa::current_isa::memory::MemoryInterfaceImpl as IsaMem;
use crate::llk::isa::interface::memory::{AddressSpaceInterface, MemoryInterface};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FreeRegion {
    start: VAddr,
    size:  usize,
}

pub enum Error {
    InsufficientPhysicalPageFramesAvailable,
    VirtualAddressSpaceLimitReached,
    InvalidAlignment,
    PhysicalMemoryError(PMemError),
}

impl From<PMemError> for Error {
    fn from(err: PMemError) -> Self {
        Error::PhysicalMemoryError(err)
    }
}

pub struct DoubleFreeListAllocator {
    heap_start: VAddr,
    heap_end: VAddr,
    heap_limit: VAddr,
    avail_by_addr: RawVec<FreeRegion>,
    avail_by_size: RawVec<FreeRegion>,
}

impl DoubleFreeListAllocator {
    pub fn new() -> Self {
        Self {
            by_addr: RawVec::try_new(),
            by_size: RawVec::try_new(),
        }
    }

    fn grow_heap(&mut self, n_pages: usize) -> Result<VAddr, Error> {
        if self.heap_limit - self.heap_end
            < (n_pages * <IsaMem as MemoryInterface>::PAGE_SIZE) as isize
        {
            return Err(Error::VirtualAddressSpaceLimitReached);
        }
        // obtain a handle to the current address space
        let mut curr_as = <IsaMem as MemoryInterface>::AddressSpace::get_current();
        let mut mapping = MemoryMapping {
            paddr: PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame()?,
            vaddr: self.heap_end,
            page_type: PageType::KernelData,
        };
    }
}
