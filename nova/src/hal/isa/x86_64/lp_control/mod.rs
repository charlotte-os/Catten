use core::arch::asm;

use crate::hal::isa::interface::lp_control::LpCtlIfce;

pub struct LpCtl;

impl LpCtlIfce for LpCtl {
    #[inline(always)]
    fn halt() -> ! {
        unsafe {
            asm!("hlt");
        }
        loop {}
    }
}
