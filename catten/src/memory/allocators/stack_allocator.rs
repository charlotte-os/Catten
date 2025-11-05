//! # Kernel Stack Allocator
//!
//! This module provides a an allocator for kernel thread stacks.
//!
//! Please note that on all supported architectures, the stack grows towards lower addresses so this
//! is the highest address of the stack. Also be aware that stacks allocated using this allocator
//! are mapped into the kernel stack arena in the higher half which means it is only suitable for
//! allocating stacks for kernel threads. Stacks are surrounded on both sides by guard pages to
//! allow for safe stack overflow detection and when enabled for the owning thread, transparent
//! reallocation such that from that thread's perspective it is as if the stack overflow never
//! happened.

use alloc::collections::BTreeSet;
use core::ops::Bound::{Excluded, Unbounded};

use spin::{Lazy, RwLock};

use super::memory;
use crate::cpu::isa::memory::paging::PAGE_SIZE;
use crate::cpu::isa::memory::{MemoryInterface, MemoryInterfaceImpl};
use crate::memory::linear::VAddr;
use crate::memory::linear::address_map::LA_MAP;
use crate::memory::{AddressSpaceInterface, KERNEL_AS};

static KERNEL_GUARD_PAGE_SET: Lazy<RwLock<BTreeSet<VAddr>>> =
    Lazy::new(|| RwLock::new(BTreeSet::new()));

pub enum Error {
    IsaMemoryIfce(<MemoryInterfaceImpl as MemoryInterface>::Error),
    AllocatorsMemory(memory::Error),
    InvalidStack,
}

impl From<<MemoryInterfaceImpl as MemoryInterface>::Error> for Error {
    fn from(err: <MemoryInterfaceImpl as MemoryInterface>::Error) -> Self {
        Error::IsaMemoryIfce(err)
    }
}

impl From<memory::Error> for Error {
    fn from(err: memory::Error) -> Self {
        Error::AllocatorsMemory(err)
    }
}

/// Allocate a kernel stack with `n_pages` being the number of usable pages.
///
/// The address returned by this function is the base address of the stack and it is
/// aligned to the page size and suitable for placing directly into the stack pointer register.
/// This is guaranteed to be the case under all supported architectures.
pub fn allocate_stack(n_pages: usize) -> Result<VAddr, Error> {
    const NUM_GUARD_PAGES: usize = 2;
    // find a suitable range in the kernel stack arena
    let stack_buf_base = KERNEL_AS.lock().find_free_region(
        n_pages + NUM_GUARD_PAGES,
        (*LA_MAP.get_region(crate::memory::linear::address_map::RegionType::KernelStackArena))
            .clone()
            .into(),
    )?;
    memory::try_allocate_and_map_range(stack_buf_base + PAGE_SIZE, n_pages)?;
    Ok(stack_buf_base + PAGE_SIZE * (n_pages + 1))
}

/// Deallocate a kernel stack previously allocated by `allocate_stack`.
pub fn deallocate_stack(stack_end: VAddr) -> Result<(), Error> {
    let n_pages = validate_stack(stack_end)?;
    memory::unmap_and_deallocate_range(stack_end - PAGE_SIZE * (n_pages + 1), n_pages);
    Ok(())
}

fn validate_stack(stack_end: VAddr) -> Result<usize, Error> {
    let stack_buf_base = stack_end - PAGE_SIZE;
    let guard_set = KERNEL_GUARD_PAGE_SET.read();
    if guard_set.contains(&stack_buf_base) {
        let next_guard = guard_set
            .range((Excluded(&stack_buf_base), Unbounded))
            .next()
            .copied()
            .ok_or(Error::InvalidStack)?;

        Ok((next_guard - stack_buf_base) as usize / PAGE_SIZE)
    } else {
        Err(Error::InvalidStack)
    }
}
