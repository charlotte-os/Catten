use crate::cpu::isa::x86_64::constants::*;

pub const APIC_TIMER_LVTR_TMM_BIT: u64 = 1 << 17;
pub const APIC_TIMER_LVTR_MASK_BIT: u64 = 1 << 16;
pub const APIC_TIMER_LVTR_VECTOR_VALUE: u64 = 255;
pub const APIC_TIMER_DIVIDER_VALUE_1: u64 = 0b1011;

pub fn init() {
    unsafe {
        // Set the divider register to indicate a divisor of 1 to use the maximum frequency
        // This is always 200 MHz on supported AMD processors and the crystal oscillator frequency on Intel processors
        // Ref: AMD APM 16.4.1
        msrs::write(msrs::APIC_TIMER_DIVIDE_CONFIGURATION, APIC_TIMER_DIVIDER_VALUE_1);
        // Clear the mode, mask, and delivery status bits and set the vector
        msrs::write(msrs::APIC_TIMER_LVTR, APIC_TIMER_LVTR_VECTOR_VALUE);
        // The APIC timer is now initialized and ready to use with x2APIC mode
    }
}
