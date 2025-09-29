pub mod gdt;

use crate::isa::interface::init::InitInterface;
use crate::isa::lp::ops::get_lp_id;
use crate::isa::lp::{init_ap, init_bsp};
use crate::logln;

pub struct IsaInitializer;

impl InitInterface for IsaInitializer {
    type Error = core::convert::Infallible;

    fn init_bsp() -> Result<(), Self::Error> {
        let lp_id = get_lp_id();
        logln!("LP{}: Starting x86-64 bootstrap processor initialization", lp_id);
        // Initialize TSS, GDT, and IDT
        init_bsp();
        logln!("LP{}: x86-64 bootstrap processor initialization complete", lp_id);
        // return success
        Ok(())
    }

    fn init_ap() -> Result<(), Self::Error> {
        let lp_id = get_lp_id();
        logln!("LP{}: Starting x86-64 application processor initialization", lp_id);
        // Initialize TSS, GDT, and IDT
        init_ap();
        logln!("LP{}: x86-64 logical processor initialization complete", lp_id);
        Ok(())
    }

    fn deinit() -> Result<(), Self::Error> {
        // Nothing to do here yet
        Ok(())
    }
}
