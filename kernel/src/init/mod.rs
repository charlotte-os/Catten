use crate::logln;
use crate::hal::isa::init::{IsaInitializer, interface::InitInterface};

pub fn kernel_init() {
    logln!("Performing ISA specific initialization...");
    match IsaInitializer::init() {
            Ok(_) => logln!("ISA specific initialization complete."),
            Err(e) => {
                    // initialization failure is irrecoverable
                    panic!("ISA specific initialization failed: {:?}", e);
            }
    }
}