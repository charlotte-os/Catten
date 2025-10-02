mod ap;
mod bsp;
pub mod gdt;

use crate::isa::interface::init::InitInterface;
use crate::isa::lp::LpControl;
use crate::isa::memory::paging::PAGE_SIZE;
use crate::logln;

const INTERRUPT_STACK_SIZE: usize = PAGE_SIZE * 4;

pub struct IsaInitializer;

impl InitInterface for IsaInitializer {
    type Error = core::convert::Infallible;

    fn init_bsp() -> Result<(), Self::Error> {
        let lp_id = LpControl::get_lp_id();
        logln!("LP{}: Starting x86-64 bootstrap processor initialization", lp_id);
        // Initialize TSS, GDT, and IDT
        bsp::init_bsp();
        logln!("LP{}: x86-64 bootstrap processor initialization complete", lp_id);
        // return success
        Ok(())
    }

    fn init_ap() -> Result<(), Self::Error> {
        let lp_id = LpControl::get_lp_id();
        logln!("LP{}: Starting x86-64 application processor initialization", lp_id);
        // Initialize TSS, GDT, and IDT
        ap::init_ap();
        logln!("LP{}: x86-64 logical processor initialization complete", lp_id);
        Ok(())
    }

    fn deinit() -> Result<(), Self::Error> {
        // Nothing to do here yet
        Ok(())
    }
}
