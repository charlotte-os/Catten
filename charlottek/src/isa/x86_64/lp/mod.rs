// x86_64 Logical Processor Operations
pub mod ops;
pub mod thread_context;

use alloc::alloc::{Layout, alloc_zeroed};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::sync::Exclusive;

use spin::{Lazy, Mutex};

use crate::cpu::multiprocessor::get_lp_count;
use crate::isa::init::gdt::*;
use crate::isa::interrupts::idt::Idt;
use crate::isa::interrupts::register_fixed_isr_gates;
use crate::isa::lp::ops::get_lp_id;
use crate::isa::memory::paging::PAGE_SIZE;
use crate::logln;

const INTERRUPT_STACK_SIZE: usize = PAGE_SIZE * 4;
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
static mut BSP_LP: MaybeUninit<LogicalProcessor> = MaybeUninit::uninit();
static LP_TABLE: Lazy<Mutex<Vec<Exclusive<LogicalProcessor>>>> =
    Lazy::new(|| Mutex::new(Vec::with_capacity(get_lp_count() as usize)));

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

pub fn init_ap() {
    let lpid = get_lp_id();
    logln!("LP{}: Allocating AP interrupt stack.", lpid);
    let int_stack = NonNull::slice_from_raw_parts(
        unsafe {
            NonNull::new(alloc_zeroed(Layout::new::<[u8; INTERRUPT_STACK_SIZE]>()))
                .expect("Failed to allocate AP interrupt stack")
        },
        INTERRUPT_STACK_SIZE,
    );
    logln!("LP{}: Allocating AP DF stack.", lpid);
    let df_stack = NonNull::slice_from_raw_parts(
        unsafe {
            NonNull::new(alloc_zeroed(Layout::new::<[u8; INTERRUPT_STACK_SIZE]>()))
                .expect("Failed to allocate AP DF stack")
        },
        INTERRUPT_STACK_SIZE,
    );
    logln!("LP{}: Allocating AP TSS.", lpid);
    let tss = unsafe {
        NonNull::new_unchecked(Box::into_raw(Box::new(Tss::new(
            int_stack.as_ptr().addr() as u64,
            df_stack.as_ptr().addr() as u64,
        ))))
    };
    logln!("LP{}: Allocating AP GDT.", lpid);
    let gdt = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(Gdt::new(tss.as_ref())))) };
    logln!("LP{}: Loading AP GDT.", lpid);
    unsafe {
        gdt.as_ref().load();
    }
    logln!("LP{}: Allocating AP IDT.", lpid);
    let mut idt = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(Idt::new()))) };
    logln!("LP{}: Registering AP ISRs.", lpid);
    register_fixed_isr_gates(unsafe { idt.as_mut() });
    logln!("LP{}: Loading AP IDT.", lpid);
    unsafe {
        idt.as_ref().load();
    }
    logln!("LP{}: Initializing AP logical processor structure.", lpid);
    let lp = LogicalProcessor::new(int_stack, df_stack, tss, gdt, idt);
    logln!(
        "LP{}: Initializing AP logical processor structure complete. Pushing to LP_TABLE.",
        lpid
    );
    LP_TABLE.lock().push(Exclusive::new(lp));
}
