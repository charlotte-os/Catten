//! ISRs with fixed vector numbers across all logical processors

pub mod context_switch;
pub mod exceptions;
pub mod ipis;
pub mod spurious;
pub mod vector_assignments;

use vector_assignments::*;

use crate::cpu::isa::init::gdt::KERNEL_CODE_SELECTOR;
use crate::cpu::isa::interrupts::idt::Idt;

pub fn register_fixed_isr_gates(idt: &mut Idt) {
    exceptions::load_exceptions(idt);
    idt.set_gate(
        SPURIOUS_INTERRUPT_VECTOR,
        spurious::isr_spurious,
        KERNEL_CODE_SELECTOR,
        false,
        true,
    );
    idt.set_gate(
        CONTEXT_SWITCH_VECTOR,
        context_switch::isr_context_switch,
        KERNEL_CODE_SELECTOR,
        false,
        true,
    );
    idt.set_gate(WAKE_LP_VECTOR, context_switch::isr_wake_lp, KERNEL_CODE_SELECTOR, false, true);
}
