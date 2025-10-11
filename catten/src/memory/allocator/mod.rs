mod memory;

use alloc::alloc::{AllocError, Allocator, Layout};
use core::ptr::{NonNull, null_mut, slice_from_raw_parts_mut};

use spinning_top::RawSpinlock;
use spinning_top::lock_api::RawMutex;

use crate::cpu::isa::memory::paging::PAGE_SIZE;
use crate::memory::VAddr;
use crate::memory::linear::address_map::{LA_MAP_48BIT, RegionType};

pub static GLOBAL_ALLOCATOR: GeneralAlloc = GeneralAlloc::new();

struct BlockDesc {
    base: VAddr,
    size: usize,
}

/// The general dynamic memory allocator of Catten
struct GeneralAlloc {
    lock: RawSpinlock,
    arena_size: usize,
    free_list_addr: *mut [*mut BlockDesc],
    free_list_size: *mut [*mut BlockDesc],
}

/// Safety: The pointers in here are never accessed from anywhere else and they are basically used
/// as manual vectors of manual Boxes since we cannot use those types given that we are the
/// allocator
unsafe impl Sync for GeneralAlloc {}

impl GeneralAlloc {
    // We use a private const constructor since this type is a singleton and should dynamically
    // provision memory and address space regions anyway
    const fn new() -> Self {
        GeneralAlloc {
            lock: RawSpinlock::INIT,
            arena_size: 0,
            free_list_addr: slice_from_raw_parts_mut(null_mut(), 0),
            free_list_size: slice_from_raw_parts_mut(null_mut(), 0),
        }
    }

    /// This function is used instead of new because this type is a singleton
    pub fn get() -> &'static Self {
        &GLOBAL_ALLOCATOR
    }

    /// What do you think it does?
    fn expand_arena(&self) -> Result<(), AllocError> {
        // This operation requires mutual exclusion
        let _lock = self.lock.lock();
        let expansion_base =
            LA_MAP_48BIT.get_region(RegionType::KernelAllocatorArena).base + self.arena_size;
        if memory::try_allocate_and_map_range(expansion_base, self.arena_size / PAGE_SIZE).is_err()
        {
            Err(AllocError)
        } else {
            Ok(())
        }
    }

    fn find_best_fit(&self) -> Option<*mut BlockDesc> {
        todo!(
            "Scan the freelist ordered by size to find the smallest free block that can meet the specified size and alignment requirements, if any."
        )
    }
}

unsafe impl Allocator for GeneralAlloc {
    fn allocate(&self, layout: core::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        todo!()
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        todo!()
    }
}
