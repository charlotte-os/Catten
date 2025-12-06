//! # x86-64 Model Specific Registers (MSRs)
//!
//! Make sure to check any necessary CPUID features before using these MSRs as not all of them are
//! architectural.
#[inline(always)]
pub unsafe fn read(msr: u32) -> u64 {
    let low: u32;
    let high: u32;

    unsafe {
        core::arch::asm!(
            "rdmsr",
            out("eax") low,
            out("edx") high,
            in("ecx") msr,
            options(nomem, nostack)
        );
    }

    ((high as u64) << 32) | (low as u64)
}

#[inline(always)]
pub unsafe fn write(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;

    unsafe {
        core::arch::asm!(
            "wrmsr",
            in("eax") low,
            in("edx") high,
            in("ecx") msr,
            options(nomem, nostack)
        );
    }
}

/// # x2APIC MSRs
/// Ref: AMD APM 16.11.1 and Intel SDM Vol.3 12.12.1.2
pub const LAPIC_ID: u32 = 0x802;
pub const APIC_EOI_REGISTER: u32 = 0x80b;
pub const APIC_SPURIOUS_INTERRUPT_VECTOR: u32 = 0x80f;
pub const INTERRUPT_COMMAND_REGISTER: u32 = 0x830;
pub const APIC_TIMER_LVTR: u32 = 0x832;
pub const APIC_TIMER_INITIAL_COUNT: u32 = 0x838;
pub const APIC_TIMER_CURRENT_COUNT: u32 = 0x839;
pub const APIC_TIMER_DIVIDE_CONFIGURATION: u32 = 0x83e;
/// # TSC_AUX MSR
pub const TSC_AUX: u32 = 0xc000_0103;
