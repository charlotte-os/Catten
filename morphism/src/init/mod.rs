use crate::common::vector::Vec;
use crate::llk::isa::current_isa::init::IsaInitializer;
use crate::llk::isa::interface::init::InitInterface;
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
    logln!("Performing ISA independent initialization...");
    logln!("Initializing physical memory...");
    match PHYSICAL_FRAME_ALLOCATOR.try_lock() {
        Some(pfa) => {
            logln!("PhysicalFrameAllocator: {:?}", pfa);
        }
        None => {
            panic!("Failed to acquire lock on PhysicalFrameAllocator.");
        }
    }
    logln!("Initializing kernel allocator...");
    match crate::memory::allocator::init_allocator() {
        Ok(_) => logln!("Kernel allocator initialized."),
        Err(_) => {
            panic!("Kernel allocator initialization failed!");
        }
    }
    logln!("Initializing the log prefix vector...");
    crate::log::LOG_PREFIX
        .lock()
        .insert(Vec::try_new().expect("Failed to initialize the log prefix vector."));
    logln!("Log prefix vector initialized.")
}
