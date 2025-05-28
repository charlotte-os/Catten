pub trait LpControlIfce {
    fn halt() -> !;
    fn mask_interrupts();
    fn unmask_interrupts();
}
