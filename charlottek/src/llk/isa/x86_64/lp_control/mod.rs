use core::arch::asm;

use crate::llk::isa::interface::lp_control::LpControlIfce;

pub struct LpControl;

impl LpControlIfce for LpControl {
    #[inline(always)]
    fn halt() -> ! {
        unsafe {
            asm!("hlt");
        }
        loop {}
    }

    #[inline(always)]
    fn mask_interrupts() {
        unsafe {
            asm!("cli");
        }
    }

    #[inline(always)]
    fn unmask_interrupts() {
        unsafe {
            asm!("sti");
        }
    }
}
