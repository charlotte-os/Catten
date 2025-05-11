use core::alloc::{Allocator, Layout};
use core::mem::transmute;

use talc::{ErrOnOom, Talc};

use crate::common::raw_mutex::RawMutex;
use crate::logln;
use crate::memory::allocator::KERNEL_ALLOCATOR;

pub fn test_allocator() {
    logln!("Starting the kernel allocator self-test...");
    logln!("Kernel allocator self-test: Allocating 1024 bytes...");
    let ptr = KERNEL_ALLOCATOR
        .lock()
        .lock::<RawMutex>()
        .allocate(Layout::from_size_align(1024, 8).unwrap())
        .unwrap();
    logln!(
        "Kernel allocator self-test: Allocated 1024 bytes at {:p}",
        ptr
    );
    let slice = unsafe { core::slice::from_raw_parts_mut(transmute(ptr), 1024) };
    logln!("Kernel allocator self-test: Writing to allocated memory...");
    for i in 0..1024 {
        slice[i] = i as u8;
    }
    logln!("Kernel allocator self-test: Write complete.");
    logln!("Kernel allocator self-test: Reading from allocated memory...");
    for i in 0..1024 {
        assert_eq!(slice[i], i as u8);
    }
    logln!("Kernel allocator self-test: Read complete.");
    logln!("Kernel allocator self-test: Deallocating allocated memory...");
    unsafe {
        KERNEL_ALLOCATOR
            .lock()
            .lock::<RawMutex>()
            .deallocate(transmute(ptr), Layout::from_size_align(1024, 8).unwrap());
    }
    logln!("Kernel allocator self-test: Deallocation complete.");
    logln!("Kernel allocator self-test: Allocating 8 KiB...");
    let ptr = KERNEL_ALLOCATOR
        .lock()
        .lock::<RawMutex>()
        .allocate(Layout::from_size_align(8192, 8).unwrap())
        .unwrap();
    logln!("Kernel allocator self-test: Allocated 8 KiB at {:p}", ptr);
    logln!("Kernel allocator self-test: Writing to allocated memory...");
    let slice = unsafe { core::slice::from_raw_parts_mut(transmute(ptr), 8192) };
    for i in 0..8192 {
        slice[i] = i as u8;
    }
    logln!("Kernel allocator self-test: Write complete.");
    logln!("Kernel allocator self-test: Reading from allocated memory...");
    for i in 0..8192 {
        assert_eq!(slice[i], i as u8);
    }
    logln!("Kernel allocator self-test: Read complete.");
    logln!("Kernel allocator self-test: Deallocating allocated memory...");
    unsafe {
        KERNEL_ALLOCATOR
            .lock()
            .lock::<RawMutex>()
            .deallocate(transmute(ptr), Layout::from_size_align(8192, 8).unwrap());
    }
    logln!("Kernel allocator self-test: Deallocation complete.");

    logln!("Kernel allocator self-test: PASSED");
}
