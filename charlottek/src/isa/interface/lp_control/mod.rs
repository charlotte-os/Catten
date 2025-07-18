use alloc::boxed::Box;

use crate::tsmp::threading::ThreadId;

pub trait LpControlIfce {
    type LpId;
    type LpState;
    type Error;

    fn halt() -> !;
    fn mask_interrupts();
    fn unmask_interrupts();
    fn get_lp_id() -> Self::LpId;
    extern "C" fn switch_context() -> !;
}
