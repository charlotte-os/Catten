use core::mem::MaybeUninit;
use core::ptr::NonNull;

use crate::llk::isa::x86_64::memory::paging::PAGE_SIZE;

struct PageLevelAllocator {
    heap_start:  VAddr,
    heap_npages: usize,
    heap_bitmap: *mut u8,
}

const SLAB_ALLOCATION_REGION_SIZE: usize = PAGE_SIZE * 16;

struct SlabAllocationRegion<const N_BYTES: usize> {
    next: Option<NonNull<SlabAllocationRegion<N_BYTES>>>,
    data: [u8; N_BYTES],d
}

struct WordLevelAllocator {
    slab_alloc_regions8: Option<NonNull<u8>>,
}
