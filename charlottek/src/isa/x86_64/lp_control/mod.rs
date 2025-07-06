use core::arch::{asm, naked_asm};

use crate::isa::interface::lp_control::LpControlIfce;

pub enum Error {}

pub struct LpControl;

impl LpControlIfce for LpControl {
    type Error = Error;
    type LpId = u32;
    type LpState = LpState;

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
                out("eax") lp_id
            );
        }
        lp_id
    }
    #[unsafe(naked)]
    extern "C" fn save_lp_state() -> Result<(), Self::Error> {
        naked_asm!(
            ""
        )
    }
    #[unsafe(naked)]
    extern "C" fn load_lp_state() -> Result<(), Self::Error> {
        todo!("Implement context loading");
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct LpState {
    gprs: [u64; 16],
    rip: u64,
    rflags: u64,
}
