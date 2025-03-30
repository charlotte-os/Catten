use core::ptr::{null_mut, NonNull};

use crate::llk::isa::interface::memory::address::{Address, VirtualAddress};
use crate::llk::isa::current_isa::memory::paging::PAGE_SIZE;
use crate::memory::vmem::VAddr;

pub enum Error {
    FreeBlockTooSmall,
    FreeBlockCannotAccomodateAlignment,
    WouldExceedHeapLimit,
}

#[derive(Debug, Clone)]
struct FreeBlock {
    prev: Option<NonNull<FreeBlock>>,
    next: Option<NonNull<FreeBlock>>,
    size: usize,
}

impl FreeBlock {
    fn allocate_from(&mut self, alignment: usize, size: usize)-> Result<VAddr, Error> {
        if self.size < size {
            return Err(Error::FreeBlockTooSmall);
        }
        // Attempt to allocate from the back of the block and shift up as needed to align the address
        todo!("Write code to allocate a buffer with the needed size and alignment starting from the back of the block.");
    }
}

#[derive(Debug)]
pub struct Allocator {
    heap_start: VAddr,
    heap_end:  VAddr,
    heap_limit: Option<VAddr>,
    free_list:  *mut FreeBlock,
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
            unsafe {
                Some(&mut *current)
            }
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

        Ok(())
    }
}
