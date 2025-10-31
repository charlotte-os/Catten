use core::arch::asm;

use spin::Mutex;

use crate::common::time::ExtDuration;
use crate::cpu::isa::interrupts::idt::Idt;
use crate::cpu::isa::interrupts::{ipis, x2apic};
use crate::cpu::scheduler::local::LocalScheduler;

pub struct LpLocalSegment {
    pub ipi_mailbox: *const ipis::IpiRpc,
    local_scheduler: Mutex<LocalScheduler>,
    idt: Idt,
    apic_timer_res: ExtDuration,
}

impl LpLocalSegment {
    fn new() -> Self {
        LpLocalSegment {
            ipi_mailbox: core::ptr::null(),
            local_scheduler: Mutex::new(LocalScheduler::new()),
            idt: Idt::new(),
            apic_timer_res: x2apic::measure_timer_res(),
        }
    }

    pub fn install() {}
}

pub extern "C" fn get_per_lp_data_segment() -> &'static LpLocalSegment {
    unsafe {
        let gs_base: u64;
        asm!(
            "rdgsbase {}", 
            out(reg) gs_base
        );
        &*(gs_base as *const LpLocalSegment)
    }
}
