//! # x86_64 Logical Processor Control Interface

pub mod core_state;
pub trait LpControlIfce {
    type LpId;
    type Error;

    /* These are all low-level naked assembly functions */
    extern "C" fn halt() -> !;
    extern "C" fn mask_interrupts();
    extern "C" fn unmask_interrupts();
    extern "C" fn get_lp_id() -> Self::LpId;
    unsafe extern "custom" fn exit_context();
    unsafe extern "custom" fn enter_context();
}
