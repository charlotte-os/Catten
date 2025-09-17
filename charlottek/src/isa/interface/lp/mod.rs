//! # x86_64 Logical Processor Control Interface

pub trait CoreStateIfce {
    extern "C" fn save(&mut self);
    extern "C" fn load(&self);
}

/*
 * The following macros are used to logical processor operations in assembly and
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
