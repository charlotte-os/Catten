use core::arch::asm;

use crate::isa::interface::lp_control::LpControlIfce;

pub enum Error {}

pub struct LpControl;

impl LpControlIfce for LpControl {
    type Error = Error;
    type LpId = u32;

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

    #[inline(always)]
    fn get_lp_id() -> Self::LpId {
        let lp_id: Self::LpId;
        unsafe {
            asm!(
                "mov ecx, 0x802",
                "rdmsr",
                out reg("eax") lp_id
            );
        }
        lp_id
    }
}
