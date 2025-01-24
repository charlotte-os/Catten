use crate::hal::isa::current_isa::memory::address::paddr::PAddr;
use crate::hal::isa::interface::memory::address::PhysicalAddress;
use crate::logln;
use crate::memory::pmem::PHYSICAL_FRAME_ALLOCATOR;

pub fn test_pmem() {
    logln!("Starting physical memory subsystem tests...");
    logln!("Attempting to allocate a physical memory frame.");

    let mut pfa_lock = PHYSICAL_FRAME_ALLOCATOR.lock();

    match pfa_lock.allocate_frame() {
        Ok(ref frame) => {
            logln!("Allocated a frame at {:?}.", frame);
            let magic_number = 0xcafebabeu32;
            logln!(
                "Writing magic number 0x{:X} to the beginning of the frame.",
                magic_number
            );
            unsafe {
                let frame_ptr = frame.into_hhdm_mut::<u32>();
                frame_ptr.write(magic_number);
                logln!("Reading back magic number from the frame: {:X}", (frame_ptr.read()));
            }
            match pfa_lock.deallocate_frame(*frame) {
                Ok(()) => {
                    logln!("Successfully deallocated frame.");
                }
                Err(e) => {
                    logln!("Failed to deallocate frame!");
                    panic!(
                        "Self-test failure: Failed to deallocate a physical memory frame at address {:?}. Error: {:?}",
                        frame, e
                    );
                }
            }
        }
        Err(e) => {
            logln!("Failed to allocate a frame!");
            panic!(
                "Self-test failure: Failed to allocate a frame from the physical frame allocator. Error: {:?}",
                e
            );
        }
    }
    logln!("All physical memory subsystem tests passed.");
}
