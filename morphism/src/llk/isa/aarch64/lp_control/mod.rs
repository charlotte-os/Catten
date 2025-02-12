use crate::llk::isa::interface::lp_control::LpCtlIfce;

pub struct LpCtl;

impl LpCtlIfce for LpCtl {
    fn halt() -> ! {
        unsafe {
            core::arch::asm!("wfi");
        }
        loop {}
    }
}
