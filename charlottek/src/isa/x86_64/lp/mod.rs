// x86_64 Logical Processor Operations
pub mod ops;
pub mod thread_context;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::sync::Exclusive;

use spin::Lazy;

use crate::cpu::multiprocessor::get_lp_count;
use crate::isa::init::gdt::*;
use crate::isa::interrupts::idt::Idt;
use crate::isa::interrupts::register_fixed_isr_gates;
use crate::isa::memory::paging::PAGE_SIZE;
use crate::logln;

pub const INTERRUPT_STACK_SIZE: usize = PAGE_SIZE * 4;
pub static mut BSP_INTERRUPT_STACK: [u8; INTERRUPT_STACK_SIZE] = [0u8; INTERRUPT_STACK_SIZE];
pub static mut BSP_DF_STACK: [u8; INTERRUPT_STACK_SIZE] = [0u8; INTERRUPT_STACK_SIZE];
pub static BSP_TSS: Lazy<Tss> = Lazy::new(|| {
    Tss::new((&raw const BSP_INTERRUPT_STACK) as u64, (&raw const BSP_DF_STACK) as u64)
});
pub static BSP_GDT: Lazy<Gdt> = Lazy::new(|| Gdt::new(&BSP_TSS));
pub static BSP_IDT: Lazy<Idt> = Lazy::new(|| {
    let mut idt = Idt::new();
    register_fixed_isr_gates(&mut idt);
    idt
});
pub static mut BSP_LP: MaybeUninit<LogicalProcessor> = MaybeUninit::uninit();
pub static LP_TABLE: Lazy<Vec<Exclusive<LogicalProcessor>>> =
    Lazy::new(|| Vec::with_capacity(get_lp_count() as usize));

pub type LpId = u32;

struct LogicalProcessor {
    interrupt_stack: NonNull<[u8]>,
    double_fault_stack: NonNull<[u8]>,
    tss: NonNull<Tss>,
    gdt: NonNull<Gdt>,
    idt: NonNull<Idt>,
}

impl LogicalProcessor {
    fn new(
        interrupt_stack: NonNull<[u8]>,
        double_fault_stack: NonNull<[u8]>,
        tss: NonNull<Tss>,
        gdt: NonNull<Gdt>,
        idt: NonNull<Idt>,
    ) -> Self {
        LogicalProcessor {
            interrupt_stack,
            double_fault_stack,
            tss,
            gdt,
            idt,
        }
    }
}

unsafe impl Send for LogicalProcessor {}

pub fn init_bsp() {
    unsafe {
        BSP_GDT.load();
        BSP_IDT.load();

        let lp = LogicalProcessor::new(
            NonNull::from(&BSP_INTERRUPT_STACK),
            NonNull::from(&BSP_DF_STACK),
            NonNull::from(&*BSP_TSS),
            NonNull::from(&*BSP_GDT),
            NonNull::from(&*BSP_IDT),
        );
        BSP_LP.as_mut_ptr().write(lp);
        logln!("BSP: x86-64 logical processor initialization complete");
    }
}

pub fn init_ap() {}
