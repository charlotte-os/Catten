//! ISRs with fixed vector numbers across all logical processors

pub mod exceptions;
pub mod ipis;
pub mod spurious;
pub mod timer;

use crate::cpu::isa::constants::interrupt_vectors::SPURIOUS_INTERRUPT_VECTOR;
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
}
