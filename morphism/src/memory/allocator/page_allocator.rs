use crate::llk::isa::x86_64::memory::paging::PAGE_SIZE;
use crate::memory::pmem::{PAddr, PHYSICAL_FRAME_ALLOCATOR};
use crate::memory::vmem::VAddr;

enum Error {
    AddressSpaceExhausted,
    PhysicalFrameUnavailable,
    InvalidAlignment,
}

pub struct PageAllocator<'heap_lt> {
    heap_base: VAddr,
    heap_npages: usize,
    heap_bitmap: RawVec<u8>,
}

impl<'l> PageAllocator<'l> {
    pub fn try_new(heap_base: VAddr, heap_npages: usize) -> Result<Self, Error> {
        let heap_map_size = heap_npages / (8 * PAGE_SIZE) + 1;
        let heap_map_pbase: PAddr;
        todo!("Allocate the heap bitmap, create the PageAllocator structure and return it.");
    }
}
