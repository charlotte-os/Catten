//! # x86_64 Logical Processor Control Interface

/*
 * The following macros are used for logical processor operations in assembly and
 * must be defined in each architecture module.
 *
 * halt!() halts the current logical processor.
 * mask_interrupts!() disables interrupts on the current logical processor.
 * unmask_interrupts!() enables interrupts on the current logical processor.
 * curr_lic_id!() evaluates to the ID of the current local interrupt controller.
 * curr_lp_id!() evaluates to the ID of the current logical processor.
 * The following type aliases must also be defined:
 * LpId: The type used for logical processor IDs.
 *
 * See the x86_64 implementation for examples.
 */

use crate::cpu::isa::interrupts::LocalIntCtlr;
use crate::cpu::isa::timers::LpTimer;

pub trait LpIsaDataIfce {
    fn new() -> Self;
    fn get() -> &'static Self;
    fn get_mut() -> &'static mut Self;
    fn get_lic(&mut self) -> &mut LocalIntCtlr;
    fn get_lp_timer(&mut self) -> &mut LpTimer;
}
