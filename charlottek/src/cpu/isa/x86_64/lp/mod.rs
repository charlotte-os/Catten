mod pcid;
pub mod thread_context;

use core::arch::asm;

pub use pcid::Pcid;

pub use crate::cpu::isa::interface::lp::LpIfce;
use crate::cpu::isa::interrupts::x2apic::id::X2ApicId;
use crate::cpu::isa::interrupts::x2apic::{self};

static TSC_AUX_MSR: u32 = 0xc000_0103u32;

pub struct LogicalProcessor;

impl LpIfce for LogicalProcessor {
    // PCID, stored in the low 12 bits of CR3
    type HwAsid = Pcid;
    // obtained from MSR 802h and MSR 80Dh
    type LicId = X2ApicId;
    // kernel assigned, stored in TSC_AUX
    type LpId = u32;

    #[inline(always)]
    fn halt() -> ! {
        //! Halts the calling processor to wait for any unmasked interrupts
        unsafe {
            asm!(
                "hlt",
                options(noreturn)
            )
        }
    }

    #[inline(always)]
    fn mask_interrupts() {
        //! Clears the interrupt enable bit in RFLAGS
        unsafe { asm!("cli") }
    }

    #[inline(always)]
    fn unmask_interrupts() {
        //! Sets the interrupt enable bit in RFLAGS
        unsafe { asm!("sti") }
    }

    #[inline(always)]
    fn read_lic_id() -> Self::LicId {
        x2apic::id::X2ApicId::get_local()
    }

    #[inline(always)]
    fn write_lp_id(lp_id: Self::LpId) {
        unsafe {
            asm!(
                "wrmsr",
                in("ecx") 0xc000_0103u32,
                in("eax") lp_id as u32,
                in("edx") 0u32,
                options(nostack, preserves_flags)
            );
        }
    }

    #[inline(always)]
    fn read_lp_id() -> Self::LpId {
        //! Obtains the kernel assigned logical processor identifier
        let mut lp_id: Self::LpId;
        unsafe {
            asm!(
                "rdpid {:r}",
                out(reg) lp_id
            );
        }
        lp_id
    }
}
