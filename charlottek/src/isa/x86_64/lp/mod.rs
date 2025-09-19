// x86_64 Logical Processor Operations

use core::arch::asm;
use core::arch::naked_asm;

pub type LpId = u32;

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
pub use halt;

#[rustfmt::skip]
#[macro_export]
macro_rules! mask_interrupts {
    () => {
        unsafe {
            asm!("cli", options(nomem, nostack, preserves_flags));
        }
    };
}
pub use mask_interrupts;

#[rustfmt::skip]
#[macro_export]
macro_rules! unmask_interrupts {
    () => {
        unsafe {
            asm!("sti", options(nomem, nostack, preserves_flags));
        }
    };
}
pub use unmask_interrupts;

#[rustfmt::skip]
#[macro_export]
macro_rules! curr_lic_id {
    () => {{
        let apic_id: u32;
        unsafe {
            core::arch::asm!(
                // In x2APIC mode, the LAPIC ID can be read from MSR 0x802
                // We require x2APIC mode to be supported and enabled so we do not check for it here
                "mov ecx, 0x802",
                "rdmsr",
                out("eax") apic_id,
                out("edx") _,
                out("ecx") _,
                options(nomem, nostack, preserves_flags)
            );
        }
        apic_id
    }};
}
pub use curr_lic_id;

#[unsafe(no_mangle)]
pub static TSC_AUX_MSR: u32 = 0xc000_0103;

pub fn store_lp_id(id: LpId) {
    let id_upper = ((id as u64) >> 32) as u32;
    let id_lower = ((id as u64) & (1 << 32) - 1) as u32;
    unsafe {
        asm!(
            "mov ecx, [rip + TSC_AUX_MSR]",
            "wrmsr",
            in("eax") id_lower,
            in("edx") id_upper,
            options(nomem, nostack, preserves_flags)
        );
    }
}

pub fn get_lp_id() -> LpId {
    let id: u32;
    unsafe {
        asm!(
            "rdtscp",
            out("eax") _,
            out("edx") _,
            out("ecx") id,
            options(nomem, nostack, preserves_flags)
        );
    }
    id
}
