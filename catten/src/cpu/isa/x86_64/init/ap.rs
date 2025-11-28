use alloc::boxed::Box;
use alloc::vec::Vec;

use spin::lazy::Lazy;

use super::{INTERRUPT_STACK_SIZE, gdt};
use crate::cpu::isa::interrupts::idt::{Idt, asm_load_idt};
use crate::cpu::isa::lp::ops::get_lp_id;
use crate::cpu::multiprocessor::get_lp_count;
use crate::logln;

static AP_INTERRUPT_STACKS: Lazy<Vec<[u8; INTERRUPT_STACK_SIZE]>> = Lazy::new(|| {
    logln!("LP{}: Computing the number of AP interrupt stacks to allocate.", (get_lp_id!()));
    let num_aps = get_lp_count() - 1; // Exclude BSP
    logln!("LP{}: Allocating {} AP interrupt stacks.", (get_lp_id!()), num_aps);
    let mut ret = Vec::<[u8; INTERRUPT_STACK_SIZE]>::with_capacity(num_aps as usize);
    for _ in 0..num_aps {
        ret.push(*(Box::new([0u8; INTERRUPT_STACK_SIZE])));
    }
    logln!("LP{}: AP interrupt stacks allocated.", (get_lp_id!()));
    ret
});

static AP_DF_STACKS: Lazy<Vec<[u8; INTERRUPT_STACK_SIZE]>> = Lazy::new(|| {
    logln!("LP{}: Computing the number of AP double fault stacks to allocate.", (get_lp_id!()));
    let num_aps = get_lp_count() - 1; // Exclude BSP
    logln!("LP{}: Allocating {} AP df stacks.", (get_lp_id!()), num_aps);
    let mut ret = Vec::<[u8; INTERRUPT_STACK_SIZE]>::with_capacity(num_aps as usize);
    for _ in 0..num_aps {
        ret.push(*(Box::new([0u8; INTERRUPT_STACK_SIZE])));
    }
    logln!("LP{}: AP df stacks allocated.", (get_lp_id!()));
    ret
});

pub static AP_TSS: Lazy<Vec<super::gdt::Tss>> = Lazy::new(|| {
    logln!("LP{}: Creating the TSS vector.", (get_lp_id!()));
    let mut tsses = Vec::new();
    logln!("LP{}: Allocating {} TSS entries.", (get_lp_id!()), (get_lp_count() - 1));
    for i in 0..(get_lp_count() - 1) {
        tsses.push(super::gdt::Tss::new(
            unsafe { (&raw const AP_INTERRUPT_STACKS[i as usize]).byte_add(INTERRUPT_STACK_SIZE) }
                as u64,
            unsafe { (&raw const AP_DF_STACKS[i as usize]).byte_add(INTERRUPT_STACK_SIZE) } as u64,
        ));
    }
    logln!("LP{}: TSS vector initialized.", (get_lp_id!()));
    tsses
});

static AP_GDTS: Lazy<Vec<super::gdt::Gdt>> = Lazy::new(|| {
    logln!("LP{}: Creating the GDT vector.", (get_lp_id!()));
    let mut gdts = Vec::new();
    logln!("LP{}: Allocating {} GDT entries.", (get_lp_id!()), (get_lp_count() - 1));
    for tss in AP_TSS.iter() {
        logln!("LP{}: Constructing and allocating a GDT", (get_lp_id!()));
        gdts.push(super::gdt::Gdt::new(tss));
    }
    logln!("LP{}: GDT vector initialized.", (get_lp_id!()));
    gdts
});

pub static AP_IDTS: Lazy<Vec<crate::cpu::isa::interrupts::idt::Idt>> = Lazy::new(|| {
    logln!("LP{}: Creating the IDT vector.", (get_lp_id!()));
    let mut idts = Vec::new();
    logln!("LP{}: Allocating {} IDT entries.", (get_lp_id!()), (get_lp_count() - 1));
    for _ in 0..(get_lp_count() - 1) {
        logln!("LP{}: Constructing and allocating an IDT", (get_lp_id!()));
        let mut idt = crate::cpu::isa::interrupts::idt::Idt::new();
        logln!("LP{}: Registering fixed interrupt gates.", (get_lp_id!()));
        crate::cpu::isa::interrupts::fixed::register_fixed_isr_gates(&mut idt);
        logln!("LP{}: Pushing the initialized IDT to the vector.", (get_lp_id!()));
        idts.push(idt);
    }
    logln!("LP{}: IDT vector initialized.", (get_lp_id!()));
    idts
});

pub static AP_IDTRS: Lazy<Vec<crate::cpu::isa::interrupts::idt::Idtr>> = Lazy::new(|| {
    logln!("LP{}: Creating the IDTR vector.", (get_lp_id!()));
    let mut idtrs = Vec::new();
    logln!("LP{}: Allocating {} IDTR entries.", (get_lp_id!()), (get_lp_count() - 1));
    for idt in AP_IDTS.iter() {
        logln!("LP{}: Constructing and allocating an IDTR", (get_lp_id!()));
        idtrs.push(crate::cpu::isa::interrupts::idt::Idtr::new(
            (size_of::<Idt>() - 1) as u16,
            idt as *const Idt as u64,
        ));
    }
    logln!("LP{}: IDTR vector initialized.", (get_lp_id!()));
    idtrs
});

pub static INIT_MUTEX: Lazy<spin::Mutex<()>> = Lazy::new(|| {
    logln!("LP{}: Creating the AP init mutex.", (get_lp_id!()));
    spin::Mutex::new(())
});

pub fn init_ap() {
    let lp_id = crate::cpu::isa::lp::ops::get_lp_id!();
    /* I'm not entirely sure why this function needs to be serialized with this mutex,
    however it triple faults without it when there are 16 or more LPs.
    Only remove it when you are certain the underlying issue has been resolved. */
    let _lock = INIT_MUTEX.lock();
    logln!("LP{}: Computing LP index.", lp_id);
    let ap_index = (lp_id - 1) as usize; // APs start from LP1
    logln!("LP{}: LP index is {}.", lp_id, ap_index);
    crate::logln!("LP{}: Initializing TSS, GDT, and IDT", lp_id);
    AP_GDTS[ap_index].load();
    unsafe {
        gdt::reload_segment_regs();
    }
    unsafe { asm_load_idt(&raw const AP_IDTRS[ap_index]) };
    crate::logln!("AP{}: x86-64 logical processor initialization complete", lp_id);
}
