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

#[rustfmt::skip]
#[macro_export]
macro_rules! get_lic_id {
    () => {{
        let apic_id: u32;
        use crate::cpu::isa::constants::*;
        unsafe {
            core::arch::asm!(
                "rdmsr",
                inlateout("ecx") msrs::LAPIC_ID => _,
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

use core::arch::asm;

use super::LpId;
use crate::cpu::isa::constants::*;
use crate::cpu::isa::interface::system_info::CpuInfoIfce;
use crate::cpu::isa::system_info::{CpuInfo, IsaExtension};

pub fn store_lp_id(id: LpId) {
    if CpuInfo::is_extension_supported(IsaExtension::Rdpid) {
        let id_upper = ((id as u64) >> 32) as u32;
        let id_lower = ((id as u64) & (1 << 32) - 1) as u32;
        unsafe {
            asm!(
                "wrmsr",
                in("eax") id_lower,
                in("edx") id_upper,
                in("ecx") msrs::TSC_AUX,
                options(nostack, preserves_flags)
            );
        }
    }
    unsafe {
        /* When in kernel context, GSBASE contains a pointer to the LogicalProcessor struct for the
        current processor.The first member of that struct is always the kernel assigned LP ID. When
        entering kernel context e.g. via an interrupt GSBASE is restored using the `swapgs`
        instruction.*/
        asm!(
            "mov gs:[0], {lp_id:r}",
            lp_id = in(reg) id
        )
    }
}
#[macro_export]
macro_rules! get_lp_id {
    () => {{
        let mut id: u32;

        use crate::cpu::isa::interface::system_info::CpuInfoIfce;
        if crate::cpu::isa::system_info::CpuInfo::is_extension_supported(
            crate::cpu::isa::system_info::IsaExtension::Rdpid
        ) {
            unsafe {
                core::arch::asm!(
                    "rdpid {:e}",
                    out(reg) id,
                );
            }
        } else {
            unsafe {
                core::arch::asm!(
                    "mov {:e}, gs:[0]",
                    out(reg) id,
                );
            }
        }
        id as crate::cpu::isa::lp::LpId
    }};
}
pub use get_lp_id;
