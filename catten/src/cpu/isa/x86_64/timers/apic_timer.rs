use crate::cpu::isa::x86_64::constants::msrs;
use crate::cpu::isa::x86_64::constants::msrs::x2apic::*;

pub const APIC_TIMER_LVTR_TMM_BIT: u64 = 1 << 17;
pub const APIC_TIMER_LVTR_MASK_BIT: u64 = 1 << 16;
/// Vector number used for timer interrupts
pub const APIC_TIMER_LVTR_VECTOR_VALUE: u64 = 33;
pub const APIC_TIMER_DIVIDER_VALUE_1: u64 = 0b1011;

pub struct ApicTimerDesc {
    resolution: u64,
}

impl ApicTimerDesc {
    pub fn init() {
        unsafe {
            // Set the divider register to indicate a divisor of 1 to use the maximum frequency
            msrs::write(APIC_TIMER_DIVIDE_CONFIGURATION, APIC_TIMER_DIVIDER_VALUE_1);
            // Clear the mode, mask, and delivery status bits and set the vector
            msrs::write(APIC_TIMER_LVTR, APIC_TIMER_LVTR_VECTOR_VALUE);
            // The APIC timer is now initialized and ready to use with x2APIC mode
        }
    }
}
