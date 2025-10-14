mod memory;

use alloc::alloc::{AllocError, Allocator, Layout};
use core::cmp::min;
use core::ptr::{self, NonNull};

use spinning_top::RawSpinlock;
use spinning_top::lock_api::RawMutex;

use crate::cpu::isa::interface::memory::address::{Address, VirtualAddress};
use crate::cpu::isa::memory::paging::PAGE_SIZE;
use crate::memory::VAddr;
use crate::memory::linear::address_map::{LA_MAP, RegionType};

pub static GENERAL_ALLOCATOR: GeneralAlloc = GeneralAlloc::new();

/// The general dynamic memory allocator of Catten
struct GeneralAlloc {
    lock: RawSpinlock,
    arena_size: usize,
    free_list: *mut [NonNull<usize>],
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
            free_list: ptr::slice_from_raw_parts_mut::<NonNull<usize>>(ptr::null_mut(), 0),
        }
    }

    /// This function is used instead of new because this type is a singleton that owns the entire
    /// kernel dynamic memory region in the higher half of all address spaces
    pub fn get() -> &'static Self {
        &GENERAL_ALLOCATOR
    }

    /// What do you think it does?
    fn expand_heap(&self) -> Result<(), AllocError> {
        // This operation requires mutual exclusion
        let _lock = self.lock.lock();
        let expansion_base =
            LA_MAP.get_region(RegionType::KernelAllocatorArena).base + self.arena_size;
        if memory::try_allocate_and_map_range(expansion_base, min(self.arena_size / PAGE_SIZE, 1))
            .is_err()
        {
            Err(AllocError)
        } else {
            Ok(())
        }
    }
}

unsafe impl Allocator for GeneralAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {}

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        todo!()
    }
}
