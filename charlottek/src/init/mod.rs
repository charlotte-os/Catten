use crate::isa::init::IsaInitializer;
use crate::isa::interface::init::InitInterface;
use crate::isa::lp;
use crate::logln;
use crate::memory::PHYSICAL_FRAME_ALLOCATOR;

pub fn bsp_init() {
    logln!("Performing ISA specific initialization...");
    match IsaInitializer::init_bsp() {
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
    logln!("ISA independent initialization complete.");
    logln!("BSP initialization complete.");
}

pub fn ap_init() {
    let lp_id = lp::ops::get_lp_id();
    logln!("Initializing LP {}...", lp_id);
    logln!("LP {}: Performing ISA specific initialization...", lp_id);
    match IsaInitializer::init_ap() {
        Ok(_) => logln!("LP {}: ISA specific initialization complete.", lp_id),
        Err(e) => {
            // initialization failure is irrecoverable
            panic!("LP {}: ISA specific initialization failed: {:?}", lp_id, e);
        }
    }
}
