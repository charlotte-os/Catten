use crate::logln;
use crate::hal::isa::current_isa::memory::address::paddr::PAddr;
use crate::memory::pmem::PHYSICAL_FRAME_ALLOCATOR;

pub fn test_pmem() {
    logln!("Starting physical memory subsystem tests...");
    logln!("Attempting to allocate a physical memory frame.");
    if let Ok(frame) = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame() {
        logln!("Allocated a frame at {:?}.", frame);
        let magic_number = 0xCAFEBABEu32;
        logln!("Writing magic number {:X} to the beginning of the frame.", magic_number);
        let frame_ptr = <PAddr as Into<*mut u32>>::into(frame.clone());
        unsafe {
            frame_ptr.write(magic_number);
            logln!("Reading back magic number from the frame: {:X}", (frame_ptr.read()));
        }
        if let Ok(()) = PHYSICAL_FRAME_ALLOCATOR.lock().deallocate_frame(frame) {
            logln!("Successfully deallocated frame.");
        } else {
            logln!("Failed to deallocate frame!");
            panic!("Self-test failure: Failed to deallocate a physical memory frame at address {:?}", frame);
        }
    } else {
        logln!("Failed to allocate a frame!");
        panic!("Self-test failure: Failed to allocate a frame from the physical frame allocator");
    }
    logln!("All physical memory subsystem tests passed.")
}