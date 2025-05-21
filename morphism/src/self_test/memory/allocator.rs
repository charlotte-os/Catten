use core::alloc::{Allocator, Layout};

use crate::logln;
use crate::memory::allocator::KERNEL_ALLOCATOR;

pub fn test_allocator() {
    logln!("Starting the kernel allocator self-test...");
    logln!("Kernel allocator self-test: Allocating 1024 bytes...");
    let ptr = KERNEL_ALLOCATOR
        .lock()
        .allocate(Layout::from_size_align(1024, 8).unwrap())
        .unwrap()
        .as_non_null_ptr();
    logln!("Kernel allocator self-test: Allocated 1024 bytes at {:p}", ptr);
    logln!("Kernel allocator self-test: Writing to allocated memory...");
    for i in 0..1024 {
        unsafe {
            ptr.add(i).write(i as u8);
        }
    }
    logln!("Kernel allocator self-test: Write complete.");
    logln!("Kernel allocator self-test: Reading from allocated memory...");
    for i in 0..1024 {
        assert_eq!(unsafe { ptr.offset(i).read() }, i as u8);
    }
    logln!("Kernel allocator self-test: Read complete.");
    logln!("Kernel allocator self-test: Deallocating allocated memory...");
    unsafe {
        KERNEL_ALLOCATOR
            .lock()
            .deallocate(ptr, Layout::from_size_align(1024, 8).unwrap());
    }
    logln!("Kernel allocator self-test: Deallocation complete.");
    logln!("Kernel allocator self-test: Allocating 8 KiB...");
    let ptr = KERNEL_ALLOCATOR
        .lock()
        .allocate(Layout::from_size_align(8192, 8).unwrap())
        .unwrap()
        .as_non_null_ptr();
    logln!("Kernel allocator self-test: Allocated 8 KiB at {:p}", ptr);
    logln!("Kernel allocator self-test: Writing to allocated memory...");
    for i in 0..8192 {
        unsafe {
            ptr.add(i).write(i as u8);
        }
    }
    logln!("Kernel allocator self-test: Write complete.");
    logln!("Kernel allocator self-test: Reading from allocated memory...");
    for i in 0..8192 {
        assert_eq!(unsafe { ptr.offset(i).read() }, i as u8);
    }
    logln!("Kernel allocator self-test: Read complete.");
    logln!("Kernel allocator self-test: Deallocating allocated memory...");
    unsafe {
        KERNEL_ALLOCATOR
            .lock()
            .deallocate(ptr, Layout::from_size_align(8192, 8).unwrap());
    }
    logln!("Kernel allocator self-test: Deallocation complete.");

    logln!("Kernel allocator self-test: PASSED");
}
