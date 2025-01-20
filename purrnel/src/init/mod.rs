use core::mem::MaybeUninit;

use limine::response::MemoryMapResponse;

use crate::hal::environment::boot_protocol::limine::MEMEORY_MAP_REQUEST;
use crate::hal::isa::current_isa::init::IsaInitializer;
use crate::hal::isa::interface::init::InitInterface;
use crate::logln;
use crate::memory::pmem::*;

pub fn kernel_init() {
    logln!("Performing ISA specific initialization...");
    match IsaInitializer::init() {
        Ok(_) => logln!("ISA specific initialization complete."),
        Err(e) => {
            // initialization failure is irrecoverable
            panic!("ISA specific initialization failed: {:?}", e);
        }
    }
    logln!("Performing ISA agnostic initialization...");
    logln!("Initializing physical memory...");
    match PHYSICAL_FRAME_ALLOCATOR.try_lock() {
        Some(pfa) => {
            logln!("PhysicalFrameAllocator: {:?}", pfa);
        }
        None => {
            panic!("Failed to acquire lock on PhysicalFrameAllocator.");
        }
    }
}
