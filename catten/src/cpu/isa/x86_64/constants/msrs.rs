//! # x86-64 Model Specific Registers (MSRs)
//!
//! !!! Make sure to check any necessary CPUID features before using these MSRs as not all of them
//! are architectural. !!!

pub type Msr = u32;

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

pub const TSC_AUX: u32 = 0xc000_0103;
pub mod x2apic {
    //! # x2APIC MSRs
    //! Ref: AMD APM 16.11.1

    use super::Msr;

    pub const X2APIC_ID_REGISTER: Msr = 0x802;
    pub const APIC_VERSION_REGISTER: Msr = 0x803;
    pub const TASK_PRIORITY_REGISTER: Msr = 0x808;
    pub const ARBITRATION_PRIORITY_REGISTER: Msr = 0x809;
    pub const PROCESSOR_PRIORITY_REGISTER: Msr = 0x80a;
    pub const END_OF_INTERRUPT_REGISTER: Msr = 0x80b;
    pub const LOGICAL_DESTINATION_REGISTER: Msr = 0x80d;
    pub const SPURIOUS_INTERRUPT_VECTOR_REGISTER: Msr = 0x80f;
    pub const IN_SERVICE_REGISTER_BASE: Msr = 0x810; // 0x810 - 0x817
    pub const TRIGGER_MODE_REGISTER_BASE: Msr = 0x818; // 0x818 - 0x81f
    pub const INTERRUPT_REQUEST_REGISTER_BASE: Msr = 0x820; // 0x820 - 0x827
    pub const ERROR_STATUS_REGISTER: Msr = 0x828;
    pub const INTERRUPT_COMMAND_REGISTER: Msr = 0x830;
    pub const TIMER_LOCAL_VECTOR_TABLE_ENTRY: Msr = 0x832;
    pub const THERMAL_LOCAL_VECTOR_TABLE_ENTRY: Msr = 0x833;
    pub const PERF_COUNTER_LOCAL_VECTOR_TABLE_ENTRY: Msr = 0x834;
    pub const LOCAL_INTERRUPT_0_LOCAL_VECTOR_TABLE_ENTRY: Msr = 0x835;
    pub const LOCAL_INTERRUPT_1_LOCAL_VECTOR_TABLE_ENTRY: Msr = 0x836;
    pub const ERROR_VECTOR_TABLE_ENTRY: Msr = 0x837;
    pub const TIMER_INITIAL_COUNT_REGISTER: Msr = 0x838;
    pub const TIMER_CURRENT_COUNT_REGISTER: Msr = 0x839;
    pub const TIMER_DIVIDE_CONFIGURATION_REGISTER: Msr = 0x83e;
    pub const SELF_IPI_REGISTER: Msr = 0x83f;
    pub const EXTENDED_APIC_FEATURE_REGISTER: Msr = 0x840;
    pub const EXTENDED_APIC_CONTROL_REGISTER: Msr = 0x841;
    pub const SPECIFIC_END_OF_INTERRUPT_REGISTER: Msr = 0x842;
    pub const INTERRUPT_ENABLE_REGISTERS_BASE: Msr = 0x848; // 0x848 - 0x84f
    pub const EXTENDED_INTERRUPT_3_DOWN_TO_0_LOCAL_VECTOR_TABLE_REGISTER_BASE: Msr = 0x850;
}
