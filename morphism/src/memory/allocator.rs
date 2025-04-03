use core::alloc::{self, AllocError, Layout};
use core::ptr::{null_mut, NonNull};

use embedded_graphics::mono_font::mapping;

use super::pmem::{self, Error as PMemError, PHYSICAL_FRAME_ALLOCATOR};
use super::vmem::{MemoryMapping, PageType};
use crate::llk::isa::current_isa::memory::paging::{AddressSpace, PAGE_SIZE};
use crate::llk::isa::current_isa::memory::Error as IsaMemoryError;
use crate::llk::isa::interface::memory::address::{self, Address, VirtualAddress};
use crate::llk::isa::interface::memory::AddressSpaceInterface;
use crate::memory::vmem::VAddr;

pub enum Error {
    FreeBlockTooSmall,
    FreeBlockCannotAccomodateAlignment,
    WouldExceedHeapLimit,
    PageFrameAllocatorError(PMemError),
    IsaMemoryError(IsaMemoryError),
}

impl From<PMemError> for Error {
    fn from(err: PMemError) -> Self {
        Error::PageFrameAllocatorError(err)
    }
}
impl From<IsaMemoryError> for Error {
    fn from(err: IsaMemoryError) -> Self {
        Error::IsaMemoryError(err)
    }
}

#[derive(Debug, Clone)]
struct FreeBlock {
    prev: Option<NonNull<FreeBlock>>,
    next: Option<NonNull<FreeBlock>>,
    size: usize,
}

impl FreeBlock {
    fn allocate_from(&mut self, alignment: usize, size: usize) -> Result<VAddr, Error> {
        if self.size < size {
            return Err(Error::FreeBlockTooSmall);
        }
        // Attempt to allocate from the back of the block and shift up as needed to align the
        // address
        todo!(
            "Write code to allocate a buffer with the needed size and alignment starting from the \
             back of the block."
        );
    }
}

#[derive(Debug)]
pub struct Allocator {
    heap_start: VAddr,
    heap_end: VAddr,
    heap_limit: Option<VAddr>,
    free_list: *mut FreeBlock,
}

impl Allocator {
    pub const unsafe fn new(heap_start: VAddr, heap_end: VAddr, heap_limit: Option<VAddr>) -> Self {
        Self {
            heap_start,
            heap_end: heap_end,
            heap_limit: heap_limit,
            free_list: null_mut(),
        }
    }

    fn largest_free_block(&self) -> Option<&mut FreeBlock> {
        let mut largest = Option::<&mut FreeBlock>::None;
        let mut current = self.free_list;
        while current.is_null() == false {
            unsafe {
                if largest.is_none() || (*current).size > largest.as_ref().clone().unwrap().size {
                    largest = Some(&mut *current);
                }
                current = (*current).next.unwrap().as_ptr();
            }
        }
        largest
    }

    fn smallest_greater_than(&self, size: usize) -> Option<&mut FreeBlock> {
        let mut sgt = Option::<&mut FreeBlock>::None;
        let mut current = self.free_list;
        while current.is_null() == false {
            unsafe {
                if (*current).size >= size {
                    if sgt.is_none() || (*current).size < sgt.as_ref().clone().unwrap().size {
                        sgt = Some(&mut *current);
                    }
                }
                current = (*current).next.unwrap().as_ptr();
            }
        }
        sgt
    }

    fn compact_free_list(&mut self) {
        let mut current = self.free_list;
        while current.is_null() == false {
            unsafe {
                let next = (*current).next.unwrap().as_ptr();
                while next.is_null() == false {
                    if (*current).size + current as usize == next as usize {
                        (*current).size += (*next).size;
                        (*current).next = (*next).next;
                    }
                }
                current = (*current).next.unwrap().as_ptr();
            }
        }
    }

    fn get_last_free_block(&self) -> Option<&mut FreeBlock> {
        if self.free_list.is_null() {
            None
        } else {
            let mut current = self.free_list;
            while current.is_null() == false {
                unsafe {
                    if (*current).next.is_none() {
                        return Some(&mut *current);
                    }
                    current = (*current).next.unwrap().as_ptr();
                }
            }
            unsafe { Some(&mut *current) }
        }
    }

    fn grow_heap(&mut self, n_pages: usize) -> Result<(), Error> {
        // Attempt to grow the heap by n_pages
        // This will require allocating a new page and updating the heap_end pointer
        // If the allocation fails, return an error
        let starting_address = self.heap_end.next_aligned_to(PAGE_SIZE);
        let new_end = starting_address + (n_pages * PAGE_SIZE);
        if new_end > self.heap_limit.unwrap_or(VAddr::MAX) {
            return Err(Error::WouldExceedHeapLimit);
        }
        let mut address_space = AddressSpace::get_current();
        for i in 0..n_pages {
            let page_addr = starting_address + (i * PAGE_SIZE);
            // Allocate the page
            // If allocation fails, return an error
            // Otherwise, add the page to the free list
            let mapping = MemoryMapping {
                vaddr: page_addr,
                paddr: PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame()?,
                page_type: PageType::KernelData,
            };
            address_space.map_page(mapping)?;
        }
        Ok(())
    }

    fn shrink_heap(&mut self) -> Result<usize, Error> {
        // If the end of the heap is free and greater than a page, shrink the heap
        // This will require unmapping the page and updating the heap_end pointer
        // If the unmapping fails, return an error
        // Otherwise, remove the page from the free list
        // Return the number of pages freed
        let mut address_space = AddressSpace::get_current();
        if let Some(end_block) = self.get_last_free_block() {
            if end_block.size < PAGE_SIZE {
                Ok(0)
            } else {
                let end_block_base = VAddr::from(addr_of!(end_block));
                let end_block_end = end_block_base + end_block.size;
                // Find the first page aligned address in the free block
                let first_page_addr = end_block_base.next_aligned_to(PAGE_SIZE);
                todo!(
                    "If the last block is free and contains at least one full page we can shrink \
                     the heap and unmap the page(s)."
                );
            }
        } else {
            // If there are no free blocks in the free list, we cannot shrink the heap
            // Return 0 pages freed as this is a normal possible outcome of this operation
            // and not an error
            Ok(0)
        }
    }
}

unsafe impl alloc::Allocator for Allocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // Attempt to allocate a buffer of the requested size and alignment
        // If the allocation fails, return an error
        // Otherwise, return the allocated buffer
        let size = layout.size();
        let alignment = layout.align();
        if size == 0 {
            return Ok(NonNull::new(0 as *mut [u8; 0]).unwrap());
        }
        todo!("Write code to allocate a buffer with the needed size and alignment.");
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        // Attempt to deallocate a buffer of the requested size and alignment
        // If the deallocation fails, return an error
        // Otherwise, return the allocated buffer
        let size = layout.size();
        let alignment = layout.align();
        if size == 0 {
            return;
        }
        todo!("Write code to deallocate a buffer with the needed size and alignment.");
    }
}
