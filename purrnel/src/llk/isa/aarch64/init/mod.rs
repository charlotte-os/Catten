use super::interrupts::load_ivt;
use crate::llk::isa::interface::init::InitInterface;
use crate::logln;

pub struct IsaInitializer;

#[derive(Debug)]
pub enum Error {
    // Error type for the aarch64 architecture
}

impl InitInterface for IsaInitializer {
    type Error = Error;

    fn init() -> Result<(), Self::Error> {
        // Initialization code for the aarch64 architecture
        logln!("Performing Aarch64 ISA specific initialization...");
        // Setup the interrupt vector table
        logln!("Loading the interrupt vector table on the BSP");
        load_ivt();
        logln!("Interrupt vector table loaded on the BSP");

        logln!("Aarch64 ISA specific initialization complete!");
        Ok(())
    }

    fn deinit() -> Result<(), Self::Error> {
        // Deinitialization code for the aarch64 architecture
        Ok(())
    }
}
