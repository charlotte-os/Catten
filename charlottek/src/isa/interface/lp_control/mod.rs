use alloc::boxed::Box;

pub trait LpControlIfce {
    type LpId;
    type LpState;
    type Error;

    fn halt() -> !;
    fn mask_interrupts();
    fn unmask_interrupts();
    fn get_lp_id() -> Self::LpId;
    /// Saves the current logical processor state and returns it as a boxed value.
    /// This function must be declared as inline(always) to ensure no register state is lost.
    fn save_lp_state() -> Box<Self::LpState>;
    /// Loads the logical processor state from a previously saved state.
    fn load_lp_state(state: Box<Self::LpState>);
}
