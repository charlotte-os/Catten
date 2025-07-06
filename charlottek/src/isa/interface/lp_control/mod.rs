use crate::tsmp::threading;

pub trait LpControlIfce {
    type LpId;
    type LpState;
    type Error;

    fn halt() -> !;
    fn mask_interrupts();
    fn unmask_interrupts();
    fn get_lp_id() -> Self::LpId;
    fn get_current_tid() -> threading::ThreadId;
    extern "C" fn save_lp_state() -> Result<(), Self::Error>;
    extern "C" fn load_lp_state() -> Result<(), Self::Error>;
}
