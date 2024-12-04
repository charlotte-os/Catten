use core::arch::asm;

pub struct LpCtl;

impl super::interface::LpCtlIfce for LpCtl {
    #[inline(always)]
    fn halt() -> ! {
        unsafe {
            asm!("hlt");
        }
        loop {}
    }
}
