//! # Low-level operations for x86_64 Logical Processors

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

pub fn get_lic_id() -> u32 {
    let apic_id: u32;
    use crate::cpu::isa::lp::msrs;
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
}

use core::arch::asm;

use super::LpId;
use crate::cpu::isa::lp::msrs;

pub fn store_lp_id(id: LpId) {
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

pub fn get_lp_id() -> LpId {
    let mut id: u32;
    unsafe {
        core::arch::asm!(
            "rdtscp",
            out("edx") _,
            out("eax") _,
            out("ecx") id,
        );
    }
    id as crate::cpu::isa::lp::LpId
}

use crate::memory::VAddr;

#[inline]
pub extern "C" fn get_lp_local_base() -> VAddr {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "rdgsbase {}",
            out(reg) ret,
            options(nomem, nostack, preserves_flags)
        );
    }
    VAddr::from(ret)
}

#[inline]
pub extern "C" fn set_lp_local_base(base: VAddr) {
    unsafe {
        core::arch::asm!(
            "wrgsbase {}",
            in(reg) <VAddr as Into<u64>>::into(base),
            options(nomem, nostack, preserves_flags)
        )
    }
}

#[inline]
pub extern "C" fn get_thread_context_ptr() -> VAddr {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "rdfsbase {}",
            out(reg) ret,
            options(nomem, nostack, preserves_flags)
        );
    }
    VAddr::from(ret)
}

#[inline]
pub unsafe extern "C" fn set_thread_context_ptr(ctx_ptr: VAddr) {
    unsafe {
        core::arch::asm!(
            "wrfsbase {}",
            in(reg) <VAddr as Into<u64>>::into(ctx_ptr),
            options(nomem, nostack, preserves_flags)
        )
    };
}
