use alloc::sync::Arc;

use spin::Mutex;

use crate::cpu::isa::interrupts::idt::Idt;
use crate::cpu::isa::interrupts::x2apic::X2Apic;
use crate::cpu::scheduler::lp_schedulers::LocalScheduler;
use crate::cpu::scheduler::threads::Thread;

pub struct LpIsaData {
    pub curr_tcb: Arc<Mutex<Thread>>,
    idt: Idt,
    local_scheduler: LocalScheduler,
    apic_handle: X2Apic,
}
