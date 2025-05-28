use crate::llk::isa::interface::lp_control::LpControlIfce;

pub struct LpControl;

impl LpControlIfce for LpControl {
    #[inline(always)]
    fn halt() -> ! {
        unsafe {
            core::arch::asm!("wfi");
        }
        loop {}
    }

    #[inline(always)]
    fn mask_interrupts() {
        unsafe {
            core::arch::asm!("cpsid i");
        }
    }

    #[inline(always)]
    fn unmask_interrupts() {
        unsafe {
            core::arch::asm!("cpsie i");
        }
    }
}
