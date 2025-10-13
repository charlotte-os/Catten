mod memory;

use alloc::alloc::{AllocError, Allocator, Layout};
use core::ptr::NonNull;

use spinning_top::RawSpinlock;
use spinning_top::lock_api::RawMutex;

use crate::cpu::isa::interface::memory::address::{Address, VirtualAddress};
use crate::cpu::isa::memory::paging::PAGE_SIZE;
use crate::memory::VAddr;
use crate::memory::linear::address_map::{LA_MAP_48BIT, RegionType};

pub static GENERAL_ALLOCATOR: GeneralAlloc = GeneralAlloc::new();

struct BlockDesc {
    size: usize,
    addr_link: Option<NonNull<BlockDesc>>,
    size_link: Option<NonNull<BlockDesc>>,
}

impl BlockDesc {
    fn new(size: usize) -> Self {
        BlockDesc {
            size,
            addr_link: None,
            size_link: None,
        }
    }

    fn try_allocate_from(&self, req: Layout) -> Option<NonNull<[u8]>> {
        if req.size() > self.size {
            None
        } else {
            let self_addr = VAddr::from_ptr(&raw const *self);
            let mut aligned_base = self_addr.next_aligned_to(req.align());
            while aligned_base + req.align() + req.size() <= self_addr + self.size {
                aligned_base = aligned_base + req.align();
            }
            let overshoot = (self_addr + self.size) - (aligned_base + req.size());

            if overshoot < 0 {
                None
            } else if overshoot >= size_of::<BlockDesc>() {
                //record the new region
            }
        }
    }
}

/// The general dynamic memory allocator of Catten
struct GeneralAlloc {
    lock: RawSpinlock,
    arena_size: usize,
    free_list_addr: Option<NonNull<BlockDesc>>,
    free_list_size: Option<NonNull<BlockDesc>>,
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
            free_list_addr: Option::None,
            free_list_size: Option::None,
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
            LA_MAP_48BIT.get_region(RegionType::KernelAllocatorArena).base + self.arena_size;
        if memory::try_allocate_and_map_range(expansion_base, self.arena_size / PAGE_SIZE).is_err()
        {
            Err(AllocError)
        } else {
            Ok(())
        }
    }

    fn find_best_fit(&self, req: &Layout) -> Option<*mut BlockDesc> {
        // This operation requires mutual exclusion
        let _lock = self.lock.lock();
        let mut best_fit: Option<*mut BlockDesc> = None;
        for &block in unsafe { &*self.free_list_size } {
            if unsafe { (*block).size } >= req.size() {
                // scan from the back starting at the last address
            }
        }
        best_fit
    }
}

unsafe impl Allocator for GeneralAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        todo!()
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        todo!()
    }
}
