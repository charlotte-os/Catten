//! Double Free List Allocator
//!
//! This module provides a kernel dynamic memory allocator implementation that uses two free list to
//! track free regions of memory. One list is sorted by address and the other by size. Allocations
//! may be of any size, however it is recommended to not use it for frequent small allocations as
//! the allocator metadata will use far more memory than the actual allocations. However since most
//! kernel allocations are expected to be anywhere from medium to gargantuan in size, this is not
//! expected to be an issue.

use core::intrinsics::size_of;

use super::pmem::{Error as PMemError, VAddr, PHYSICAL_FRAME_ALLOCATOR};
use super::vmem::{MemoryMapping, PageType};
use crate::common::raw_ds::raw_vec::RawVec;
use crate::llk::isa::current_isa::memory::MemoryInterfaceImpl as IsaMem;
use crate::llk::isa::interface::memory::{AddressSpaceInterface, MemoryInterface};
use crate::llk::isa::x86_64::memory::paging::PAGE_SIZE;
use crate::memory::pmem::PAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FreeRegion {
    start: VAddr,
    size:  usize,
}

struct FreeList {
    by_addr: *mut FreeRegion,
    by_size: *mut FreeRegion,
    len: usize,
    pages_per_list: usize,
}

enum FreeListError {
    ListsFull,
    ListsEmpty,
    IndexOutOfRange,
}

impl FreeList {
    fn new(
        by_addr_buf: *mut FreeRegion,
        by_size_buf: *mut FreeRegion,
        pages_per_list: usize,
    ) -> Self {
        FreeList {
            by_addr: by_addr_buf,
            by_size: by_size_buf,
            len: 0,
            pages_per_list,
        }
    }

    fn is_full(&self) -> bool {
        self.len == self.pages_per_list * PAGE_SIZE / size_of::<FreeRegion>()
    }

    unsafe fn shift_forward_elements_at(mut list: *mut FreeList, len: usize, index: usize) -> Result<(), FreeListError> {
        if index >= list.len {
            Err(FreeListError::IndexOutOfRange)
        } else {
            unsafe {
                for i in index..len.rev() {
                    list.offset(i as isize + 1).write(list.offset(i as isize).read());
                }
                list.offset(index as isize).write(FreeRegion::default());
                Ok(())
            }
        }
    }

    unsafe fn insert_at(
        mut ptr: *mut FreeRegion,
        len: usize,
        index: usize,
        new_region: FreeRegion,
    ) -> Result<(), FreeListError> {
        if index >= self.len {
            Err(FreeListError::IndexOutOfRange)
        } else {
            shift_forward_elements_at(ptr, len, index)?;
            ptr.offset(index as isize).write(new_region);
            Ok(())
        }
    }

    fn insert_by_addr(&mut self, region: FreeRegion) -> Result<(), FreeListError> {
        if self.is_full() {
            Err(FreeListError::ListsFull)
        } else {
            let index = self
                .by_addr
                .iter()
                .position(|r| r.start > region.start)
                .unwrap_or(self.len);
            unsafe { Self::insert_at(self.by_addr.as_mut_ptr(), index, region) }
        }
    }

    fn insert_by_size(&mut self, region: FreeRegion) -> Result<(), FreeListError> {
        if self.is_full() {
            Err(FreeListError::ListsFull)
        } else {
            let index = self
                .by_size
                .iter()
                .position(|r| r.size > region.size)
                .unwrap_or(self.len);
            unsafe { Self::insert_at(self.by_size.as_mut_ptr(), index, region) }
        }
    }

    fn insert(&mut self, region: FreeRegion) -> Result<(), FreeListError> {
        if self.is_full() {
            Err(FreeListError::ListsFull)
        } else if self.len == 0 {
            self.by_addr[0] = region;
            self.by_size[0] = region;
            self.len += 1;
            Ok(())
        } else {
            self.insert_by_addr(region);
            self.insert_by_size(region);
            self.len += 1;
            Ok(())
        }
    }

    fn remove_by
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
    available: FreeList,
}

impl DoubleFreeListAllocator {
    pub fn new() -> Self {}

    fn grow_heap(&mut self, n_pages: usize) -> Result<VAddr, Error> {
        if self.heap_limit - self.heap_end
            < (n_pages * <IsaMem as MemoryInterface>::PAGE_SIZE) as isize
        {
            return Err(Error::VirtualAddressSpaceLimitReached);
        }
        // obtain a handle to the current address space
        let mut curr_as = <IsaMem as MemoryInterface>::AddressSpace::get_current();
        // create a mapping to use for the new pages
        let mut mapping = MemoryMapping {
            paddr: PAddr::from(0),
            vaddr: VAddr::from(0),
            page_type: PageType::KernelData,
        };
        // save the current heap end address since that is the beginning of the new free region
        let ret = self.heap_end;
        // allocate physical frames and map them to virtual address space apertures
        for _ in 0..n_pages {
            mapping.paddr = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame()?;
            mapping.vaddr = self.heap_end;
            curr_as.map(mapping)?;
            self.heap_end += <IsaMem as MemoryInterface>::PAGE_SIZE;
        }
        Ok(ret)
    }
}
