use super::interrupts::load_ivt;
use crate::cpu::isa::interface::init::InitInterface;
use crate::logln;

pub struct IsaInitializer;

#[derive(Debug)]
pub enum Error {
    // Error type for the aarch64 architecture
}

impl InitInterface for IsaInitializer {
    type Error = Error;

    fn init_bsp() -> Result<(), Self::Error> {
        init_ap()
    }

    fn init_ap() -> Result<(), Self::Error> {
        // Initialization code for the aarch64 architecture
        logln!("Performing Aarch64 ISA specific initialization...");
        // Setup the interrupt vector table
        logln!("Loading the interrupt vector table on the AP");
        load_ivt();
        logln!("Interrupt vector table loaded on the AP");

        logln!("Aarch64 ISA specific initialization complete!");
        Ok(())
    }

    fn deinit() -> Result<(), Self::Error> {
        // Deinitialization code for the aarch64 architecture
        Ok(())
    }
}
