use crate::isa::target::lp;
pub trait InterruptManagerIfce {
    type Error;
    type Ipi;

    fn init_interrupt_structures() -> Result<(), Self::Error>;
    fn send_ipi(lp_list: &[LpControl::LpId], ipi: Self::Ipi);
}
