// x86_64 Logical Processor Operations

use core::arch::asm;
use core::arch::naked_asm;

pub type LpId = u32;

#[rustfmt::skip]
#[macro_export]
macro_rules! halt {
    () => {
        unsafe {
            asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    };
}

#[rustfmt::skip]
#[macro_export]
macro_rules! mask_interrupts {
    () => {
        unsafe {
            asm!("cli", options(nomem, nostack, preserves_flags));
        }
    };
}

#[rustfmt::skip]
#[macro_export]
macro_rules! unmask_interrupts {
    () => {
        unsafe {
            asm!("sti", options(nomem, nostack, preserves_flags));
        }
    };
}

#[rustfmt::skip]
#[macro_export]
macro_rules! curr_lic_id {
    () => {{
        let apic_id: u32;
        unsafe {
            asm!(
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
