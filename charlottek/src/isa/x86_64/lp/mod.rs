// x86_64 Logical Processor Operations
pub mod ops;
pub mod thread_context;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::mem::MaybeUninit;

use spin::Mutex;

use crate::isa::init::gdt::*;
use crate::isa::interrupts::idt::Idt;
use crate::isa::interrupts::register_fixed_isr_gates;
use crate::isa::memory::paging::PAGE_SIZE;
use crate::logln;

pub const INTERRUPT_STACK_SIZE: usize = PAGE_SIZE * 4;
pub static BSP_INTERRUPT_STACK: [u8; INTERRUPT_STACK_SIZE] = [0u8; INTERRUPT_STACK_SIZE];
pub static mut BSP_LP_TEMP: MaybeUninit<LogicalProcessor> = MaybeUninit::uninit();
pub static LP_TABLE: Mutex<Vec<LogicalProcessor>> = Mutex::new(Vec::new());

pub type LpId = u32;

pub struct LogicalProcessor {
    tss: Tss,
    gdt: Gdt,
    interrupt_stack: Box<[u8]>,
    idt: Idt,
}

impl LogicalProcessor {
    pub fn new(interrupt_stack: Box<[u8]>) -> Self {
        let interrupt_stack_addr = interrupt_stack.as_ptr() as u64;
        let tss = Tss::new(interrupt_stack_addr + interrupt_stack.len() as u64);
        let gdt = Gdt::new(&tss);
        LogicalProcessor {
            tss,
            gdt,
            interrupt_stack,
            idt: Idt::new(),
        }
    }

    pub fn init(&mut self) {
        self.gdt.load();
        logln!("LP{}: Loaded GDT and TSS", (ops::get_lp_id()));
        Gdt::reload_segment_regs();
        logln!("LP{}: Segment registers reloaded", (ops::get_lp_id()));
        register_fixed_isr_gates(&mut self.idt);
        logln!("LP{}: Fixed ISR gates loaded", (ops::get_lp_id()));
        self.idt.load();
        logln!("LP{}: IDT loaded", (ops::get_lp_id()));
    }
}
