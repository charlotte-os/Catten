pub trait LpControlIfce {
    type LpId;
    type LpState;
    type Error;

    fn halt() -> !;
    fn mask_interrupts();
    fn unmask_interrupts();
    fn get_lp_id() -> Self::LpId;
    extern "C" fn save_lp_state() -> Result<(), Self::Error>;
    extern "C" fn load_lp_state() -> Result<(), Self::Error>;
}
