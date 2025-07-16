use crate::isa::current_isa::lp_control::LpControl;
use crate::isa::interface::lp_control::LpControlIfce;

pub trait InterruptManagerIfce {
    type Error;
    type Ipi;

    fn init_interrupt_structures() -> Result<(), Self::Error>;
    fn send_ipi(lp_list: &[LpControl::LpId], ipi: Self::Ipi);
}
