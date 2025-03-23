use core::alloc;

use talc::{ErrOnOom, Span, Talc};

use crate::llk::isa::interface::memory::address::VirtualAddress;
use crate::memory::pmem::VAddr;

pub struct Allocator {
    arena: (VAddr, VAddr),
    talc:  Talc<ErrOnOom>,
}

impl Allocator {
    /// This function creates a new kernel allocator
    /// Preconditions: All of the addresses between arena_start and arena_end are unused in the kernel address space
    /// Postconditions: A kernel allocator is created with the specified allocation arena
    /// Safety: The specified arena must not be used or accessed by another allocator or any other part of the kernel
    /// in a manner unknown to this allocator
    unsafe fn new(arena_start: VAddr, arena_end: VAddr) -> Self {
        let mut talc = Talc::new(ErrOnOom);
        talc.claim(Span::new(arena_start.into_mut(), arena_end.into_mut()))
            .expect("Error intializing talc allocator");
        Allocator {
            arena: (arena_start, arena_end),
            talc:  talc,
        }
    }
}

impl alloc::Allocator for Allocator {
    fn allocate(&self, layout: alloc::Layout) -> Result<core::ptr::NonNull<[u8]>, alloc::AllocError> {
        
    }
}
