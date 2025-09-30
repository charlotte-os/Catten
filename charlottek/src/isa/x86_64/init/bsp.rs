use spin::Lazy;

use super::INTERRUPT_STACK_SIZE;
use super::gdt::*;
use crate::isa::interrupts::idt::Idt;
use crate::isa::interrupts::register_fixed_isr_gates;
use crate::logln;

static mut BSP_INTERRUPT_STACK: [u8; INTERRUPT_STACK_SIZE] = [0u8; INTERRUPT_STACK_SIZE];
static mut BSP_DF_STACK: [u8; INTERRUPT_STACK_SIZE] = [0u8; INTERRUPT_STACK_SIZE];
static BSP_TSS: Lazy<Tss> = Lazy::new(|| {
    Tss::new((&raw const BSP_INTERRUPT_STACK) as u64, (&raw const BSP_DF_STACK) as u64)
});
static BSP_GDT: Lazy<Gdt> = Lazy::new(|| Gdt::new(&BSP_TSS));
static BSP_IDT: Lazy<Idt> = Lazy::new(|| {
    let mut idt = Idt::new();
    register_fixed_isr_gates(&mut idt);
    idt
});

pub fn init_bsp() {
    BSP_GDT.load();
    unsafe {
        reload_segment_regs();
    }
    BSP_IDT.load();
    logln!("BSP: x86-64 logical processor initialization complete");
}
