//! # Stack Buffer
//!
//! This module provides a stack buffer type that ensures proper alignment and size for stack usage.

use alloc::alloc::{Layout, alloc_zeroed, dealloc};
use core::hint::unlikely;

use crate::cpu::isa::interface::memory::address::VirtualAddress;
use crate::cpu::isa::memory::paging::PAGE_SIZE;
use crate::memory::VAddr;

/* We page align all stacks and round their sizes up to the nearest full page to simplify paging */
const STACK_ALIGNMENT: usize = PAGE_SIZE;
const MIN_STACK_SIZE: usize = PAGE_SIZE * 4;
/// A stack buffer with a top address and a size in bytes
#[derive(Debug)]
pub struct StackBuf {
    top:  VAddr,
    size: usize,
}

pub enum Error {
    OutOfMemory,
}

impl StackBuf {
    /// Adjust the size to be at least MIN_STACK_SIZE and a multiple of PAGE_SIZE
    fn fix_size(mut size: usize) -> usize {
        if unlikely(size < MIN_STACK_SIZE) {
            size = MIN_STACK_SIZE;
        }
        if unlikely(size % PAGE_SIZE != 0) {
            size = (size / PAGE_SIZE + 1) * PAGE_SIZE;
        }
        size
    }

    /// Create a new stack with the given size in bytes
    /// The size will be rounded up to the nearest page size and aligned to a page boundary
    pub fn try_new(mut size: usize) -> Result<Self, Error> {
        size = Self::fix_size(size);
        // allocate the stack memory with the correct alignment
        let buf = unsafe { alloc_zeroed(Layout::from_size_align(size, STACK_ALIGNMENT).unwrap()) };
        // return the result
        if buf.is_null() {
            Err(Error::OutOfMemory)
        } else {
            Ok(StackBuf {
                top: VAddr::from_mut(unsafe { buf.byte_add(size) }),
                size,
            })
        }
    }

    /// Attempts to resize the stack to the new size in bytes
    /// The size will be rounded up to the nearest page size and aligned to a page boundary as
    /// always
    pub fn try_resize(&mut self, new_size: usize) -> Result<(), Error> {
        let new_size = Self::fix_size(new_size);
        if new_size == self.size {
            Ok(())
        } else {
            let buf = unsafe {
                alloc_zeroed(Layout::from_size_align(new_size, STACK_ALIGNMENT).unwrap())
            };
            if buf.is_null() {
                Err(Error::OutOfMemory)
            } else {
                let self_bottom = unsafe { self.top.into_mut::<u8>().byte_sub(self.size) };
                unsafe {
                    // copy the old stack data to the new stack, up to the minimum of the two sizes
                    core::ptr::copy_nonoverlapping(
                        self_bottom,
                        buf,
                        core::cmp::min(self.size, new_size),
                    );
                    // deallocate the old stack buffer
                    dealloc(
                        self_bottom,
                        Layout::from_size_align(self.size, STACK_ALIGNMENT).unwrap(),
                    );
                }
                // update the stack to store the values for the new buffer
                self.top = VAddr::from_mut(unsafe { buf.byte_add(new_size) });
                self.size = new_size;
                Ok(())
            }
        }
    }

    /// Get the top address of the stack
    pub extern "C" fn top(&self) -> VAddr {
        self.top
    }
}

impl Drop for StackBuf {
    fn drop(&mut self) {
        let self_bottom = unsafe { self.top.into_mut::<u8>().byte_sub(self.size) };
        unsafe {
            dealloc(self_bottom, Layout::from_size_align(self.size, STACK_ALIGNMENT).unwrap());
        }
    }
}
