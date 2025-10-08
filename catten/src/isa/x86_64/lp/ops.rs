#[rustfmt::skip]
#[macro_export]
macro_rules! halt {
    () => {
        loop {
            unsafe {
                core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
            }
        }
    };
}
#[rustfmt::skip]
pub use halt;

#[rustfmt::skip]
#[macro_export]
macro_rules! mask_interrupts {
    () => {
        unsafe {
            asm!("cli", options(nomem, nostack));
        }
    };
}
#[rustfmt::skip]
pub use mask_interrupts;

#[rustfmt::skip]
#[macro_export]
macro_rules! unmask_interrupts {
    () => {
        unsafe {
            asm!("sti", options(nomem, nostack));
        }
    };
}
#[rustfmt::skip]
pub use unmask_interrupts;

pub const LAPIC_ID_MSR: u32 = 0x802;

#[rustfmt::skip]
#[macro_export]
macro_rules! get_lic_id {
    () => {{
        let apic_id: u32;
        use crate::isa::lp::ops::LAPIC_ID_MSR;
        unsafe {
            core::arch::asm!(
                "rdmsr",
                inlateout("ecx") LAPIC_ID_MSR => _,
                lateout("eax") apic_id,
                lateout("edx") _,
                options(nostack, preserves_flags)
            );
        }
        apic_id
    }};
}
#[rustfmt::skip]
pub use get_lic_id;

use core::arch::{asm, naked_asm};

use super::LpId;

pub const TSC_AUX_MSR: u32 = 0xc000_0103;

pub fn store_lp_id(id: LpId) {
    let id_upper = ((id as u64) >> 32) as u32;
    let id_lower = ((id as u64) & (1 << 32) - 1) as u32;
    unsafe {
        asm!(
            "wrmsr",
            in("eax") id_lower,
            in("edx") id_upper,
            in("ecx") TSC_AUX_MSR,
            options(nostack, preserves_flags)
        );
    }
}
#[macro_export]
macro_rules! get_lp_id {
    () => {{
        let mut id: u32;
        unsafe {
            core::arch::asm!(
                "rdpid rax",
                out("eax") id,
            );
        }
        id as crate::isa::lp::LpId
    }};
}
pub use get_lp_id;
