//! Stack Memory Management for x86_64
//!
//! This module provides functions to allocate, deallocate, reallocate, stack buffers and get the
//! top address of one since x86_64 stacks grow downwards.

use alloc::slice;

use crate::cpu::isa::memory::paging::PAGE_SIZE;
/// Allocates a zeroed stack buffer with the given number of pages aligned to PAGE_SIZE.
pub extern "C" fn allocate_stack(page_count: usize) -> *mut [u8] {
    let layout =
        core::alloc::Layout::from_size_align(page_count * super::paging::PAGE_SIZE, PAGE_SIZE)
            .expect("Invalid stack layout");
    let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
    if ptr.is_null() {
        panic!("Failed to allocate stack");
    }
    unsafe { slice::from_raw_parts_mut(ptr, page_count * super::paging::PAGE_SIZE) }
}
/// Deallocates a stack buffer previously allocated with `allocate_stack`.
pub extern "C" fn deallocate_stack(stack: *mut [u8], page_count: usize) {
    let layout =
        core::alloc::Layout::from_size_align(page_count * super::paging::PAGE_SIZE, PAGE_SIZE)
            .expect("Invalid stack layout");
    unsafe { alloc::alloc::dealloc(stack.as_mut_ptr() as *mut u8, layout) };
}
/// Returns the top address of the stack buffer. This is the address where the stack pointer
/// should be initialized to since x86_64 stacks grow downwards.
pub extern "C" fn stack_top(stack: &mut [u8]) -> *mut u8 {
    unsafe { stack.as_mut_ptr().add(stack.len()) }
}
/// Reallocates a stack buffer to a new size, copying the contents of the old stack to the new
/// stack. The old stack is deallocated. If the new size is smaller than the old size, the
/// contents are truncated. If memory allocation fails, this function preserves the old stack as is.
pub extern "C" fn reallocate_stack(
    old_stack: *mut [u8],
    old_page_count: usize,
    new_page_count: usize,
) -> *mut [u8] {
    let new_layout =
        core::alloc::Layout::from_size_align(new_page_count * super::paging::PAGE_SIZE, PAGE_SIZE)
            .expect("Invalid stack layout");
    let new_ptr = unsafe { alloc::alloc::alloc_zeroed(new_layout) };
    if new_ptr.is_null() {
        return old_stack;
    }
    let new_stack =
        unsafe { slice::from_raw_parts_mut(new_ptr, new_page_count * super::paging::PAGE_SIZE) };
    let copy_size = core::cmp::min(old_page_count, new_page_count) * super::paging::PAGE_SIZE;
    unsafe {
        core::ptr::copy_nonoverlapping(old_stack.as_mut_ptr(), new_stack.as_mut_ptr(), copy_size);
    }
    deallocate_stack(old_stack, old_page_count);
    new_stack
}
