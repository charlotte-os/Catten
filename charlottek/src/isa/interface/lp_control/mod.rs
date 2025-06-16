pub trait LpControlIfce {
    type LpId;
    type Error;

    fn halt() -> !;
    fn mask_interrupts();
    fn unmask_interrupts();
    fn get_lp_id() -> Self::LpId;
}
