use crate::isa::current_isa::init::IsaInitializer;
use crate::isa::interface::init::InitInterface;
use crate::logln;
use crate::memory::pmem::*;
use crate::environment::boot_protocol::limine::MP;

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
    logln!("Intialized kernel allocator.");
    logln!("Starting secondary processors...");
    
    }
}
