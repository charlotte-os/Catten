use alloc::sync::Arc;
use core::arch::asm;

use spin::Mutex;

use crate::cpu::isa::interrupts::idt::Idt;
use crate::cpu::isa::interrupts::ipis;
use crate::cpu::isa::timers::apic_timer::ApicTimerDesc;
use crate::cpu::scheduler::local::LocalScheduler;

pub struct LpLocalSegment {
    pub ipi_mailbox: *const ipis::IpiRpc,
    local_scheduler: Mutex<LocalScheduler>,
    idt: Idt,
    apic_timer_desc: ApicTimerDesc,
}

impl LpLocalSegment {
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
