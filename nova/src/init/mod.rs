use crate::hal::isa::interface::init::InitInterface;
use crate::hal::isa::current_isa::init::IsaInitializer;
use crate::logln;

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
