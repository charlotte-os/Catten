pub mod exceptions;
pub mod idt;
pub mod ipis;

use idt::*;
use ipis::isr_ipi;
use spin::Mutex;

use crate::isa::init::gdt;

pub static IDT: Mutex<Idt> = Mutex::new(Idt::new());

pub fn load_fixed_isr_gates(idt: &mut Idt) {
    exceptions::load_exceptions(idt);
    idt.set_gate(32, isr_ipi, gdt::KERNEL_CODE_SELECTOR, false, true);
}
