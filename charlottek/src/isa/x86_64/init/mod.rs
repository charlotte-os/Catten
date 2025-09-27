pub mod gdt;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::borrow::BorrowMut;
// core
use core::ptr;

use exceptions::load_exceptions;
// crate local
use gdt::{Gdt, Tss};
// external crates
use lazy_static::lazy_static;
use spin::Mutex;

use crate::isa::interface::init::InitInterface;
use crate::isa::interface::lp;
use crate::isa::lp::ops::get_lp_id;
use crate::isa::lp::{INTERRUPT_STACK_SIZE, LP_TABLE, LogicalProcessor};
use crate::isa::memory::paging::PAGE_SIZE;
use crate::isa::x86_64::interrupts::*;
use crate::logln;

pub struct IsaInitializer;

impl InitInterface for IsaInitializer {
    type Error = core::convert::Infallible;

    fn init_bsp() -> Result<(), Self::Error> {
        // return success
        Ok(())
    }

    fn init_ap() -> Result<(), Self::Error> {
        let lp_id = get_lp_id();
        logln!("LP{}: Starting x86-64 logical processor initialization", lp_id);
        let mut lp_table = LP_TABLE.lock();
        let lp = lp_table[lp_id as usize].borrow_mut();
        *lp = LogicalProcessor::new(
            Vec::<u8>::with_capacity(INTERRUPT_STACK_SIZE).into_boxed_slice(),
        );
        lp.init();
        logln!("LP{}: x86-64 logical processor initialization complete", lp_id);
        Ok(())
    }

    fn deinit() -> Result<(), Self::Error> {
        // Nothing to do here yet
        Ok(())
    }
}
