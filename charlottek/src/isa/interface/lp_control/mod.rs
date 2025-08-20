//! # x86_64 Logical Processor Control Interface
pub trait LpControlIfce {
    type LpId;
    type Error;

    /* These are all low-level naked assembly functions */
    extern "C" fn halt() -> !;
    extern "C" fn mask_interrupts();
    extern "C" fn unmask_interrupts();
    extern "C" fn get_lp_id() -> Self::LpId;
    extern "C" fn switch_context();
    extern "C" fn load_context();

    fn init_new_thread_stack(stack: &mut [u8], entry_point: fn() -> !, user_mode: bool);
}
