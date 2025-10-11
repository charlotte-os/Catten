pub mod context_switch;
pub mod exceptions;
pub mod idt;
pub mod ipis;
pub mod x2apic;

use context_switch::isr_switch_thread_context;
use idt::*;
use ipis::isr_interprocessor_interrupt;
use spin::Mutex;

use crate::isa::init::gdt;

pub static IDT: Mutex<Idt> = Mutex::new(Idt::new());

pub fn register_fixed_isr_gates(idt: &mut Idt) {
    exceptions::load_exceptions(idt);
    idt.set_gate(32, isr_switch_thread_context, gdt::KERNEL_CODE_SELECTOR, false, true);
    idt.set_gate(33, isr_interprocessor_interrupt, gdt::KERNEL_CODE_SELECTOR, false, true);
}
