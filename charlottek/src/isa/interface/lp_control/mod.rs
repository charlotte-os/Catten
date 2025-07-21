//! # x86_64 Logical Processor Control Interface
pub trait LpControlIfce {
    type LpId;
    type Error;

    fn halt() -> !;
    fn mask_interrupts();
    fn unmask_interrupts();
    fn get_lp_id() -> Self::LpId;
    extern "C" fn switch_context();
    extern "C" fn load_context();
}
