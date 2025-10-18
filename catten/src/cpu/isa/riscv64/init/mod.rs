use crate::cpu::isa::interface::init::InitInterface;
use crate::cpu::isa::lp::ops::get_lp_id;
use crate::cpu::isa::memory::paging::PAGE_SIZE;
use crate::logln;

const INTERRUPT_STACK_SIZE: usize = PAGE_SIZE * 4;

pub struct IsaInitializer;

impl InitInterface for IsaInitializer {
    type Error = core::convert::Infallible;

    fn init_bsp() -> Result<(), Self::Error> {
        let lp_id = get_lp_id!();
        logln!("LP{}: Starting RISC-V 64-bit bootstrap processor initialization", lp_id);
        // TODO: Initialize RISC-V specific structures (trap handlers, CSRs, etc.)
        logln!("LP{}: RISC-V 64-bit bootstrap processor initialization complete", lp_id);
        // return success
        Ok(())
    }

    fn init_ap() -> Result<(), Self::Error> {
        let lp_id = get_lp_id!();
        logln!("LP{}: Starting RISC-V 64-bit application processor initialization", lp_id);
        // TODO: Initialize RISC-V specific structures (trap handlers, CSRs, etc.)
        logln!("LP{}: RISC-V 64-bit logical processor initialization complete", lp_id);
        Ok(())
    }

    fn deinit() -> Result<(), Self::Error> {
        // Nothing to do here yet
        Ok(())
    }
}
