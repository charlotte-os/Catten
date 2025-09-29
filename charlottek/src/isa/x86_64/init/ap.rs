use alloc::vec::Vec;

use spin::lazy::Lazy;

use super::INTERRUPT_STACK_SIZE;
use crate::cpu::multiprocessor::get_lp_count;

static AP_INTERRUPT_STACKS: Lazy<Vec<[u8; INTERRUPT_STACK_SIZE]>> = Lazy::new(|| {
    let num_aps = get_lp_count() - 1; // Exclude BSP
    alloc::vec![[0u8; INTERRUPT_STACK_SIZE]; num_aps as usize]
});

static AP_DF_STACKS: Lazy<Vec<[u8; INTERRUPT_STACK_SIZE]>> = Lazy::new(|| {
    let num_aps = get_lp_count() - 1; // Exclude BSP
    alloc::vec![[0u8; INTERRUPT_STACK_SIZE]; num_aps as usize]
});

static AP_TSS: Lazy<Vec<super::gdt::Tss>> = Lazy::new(|| {
    let mut tsses = Vec::new();
    for i in 0..(get_lp_count() - 1) {
        tsses.push(super::gdt::Tss::new(
            unsafe { (&raw const AP_INTERRUPT_STACKS[i as usize]).byte_add(INTERRUPT_STACK_SIZE) }
                as u64,
            unsafe { (&raw const AP_DF_STACKS[i as usize]).byte_add(INTERRUPT_STACK_SIZE) } as u64,
        ));
    }
    tsses
});

static AP_GDTS: Lazy<Vec<super::gdt::Gdt>> = Lazy::new(|| {
    let mut gdts = Vec::new();
    for tss in AP_TSS.iter() {
        gdts.push(super::gdt::Gdt::new(tss));
    }
    gdts
});

static AP_IDTS: Lazy<Vec<crate::isa::interrupts::idt::Idt>> = Lazy::new(|| {
    let mut idts = Vec::new();
    for _ in 0..(get_lp_count() - 1) {
        let mut idt = crate::isa::interrupts::idt::Idt::new();
        crate::isa::interrupts::register_fixed_isr_gates(&mut idt);
        idts.push(idt);
    }
    idts
});

pub fn init_ap() {
    let lp_id = crate::isa::lp::ops::get_lp_id();
    let ap_index = (lp_id - 1) as usize; // APs start from LP1
    AP_GDTS[ap_index].load();
    AP_IDTS[ap_index].load();
    crate::logln!("AP{}: x86-64 logical processor initialization complete", lp_id);
}
