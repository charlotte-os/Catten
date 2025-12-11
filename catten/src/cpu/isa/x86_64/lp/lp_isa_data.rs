use alloc::boxed::Box;
use alloc::sync::Arc;

use spin::Mutex;

use crate::cpu::isa::interface::interrupts::LocalIntCtlrIfce;
use crate::cpu::isa::interface::lp::LpIsaDataIfce;
use crate::cpu::isa::interrupts::LocalIntCtlr;
use crate::cpu::isa::interrupts::idt::Idt;
use crate::cpu::isa::interrupts::x2apic::X2Apic;
use crate::cpu::isa::timers::LpTimer;
use crate::cpu::isa::timers::apic_timer::ApicTimer;
use crate::cpu::scheduler::lp_schedulers::LocalScheduler;
use crate::cpu::scheduler::lp_schedulers::strategy::RoundRobin;
use crate::cpu::scheduler::threads::Thread;

pub struct LpIsaData {
    pub curr_tcb: Option<Arc<Mutex<Thread>>>,
    idt: Idt,
    local_scheduler: LocalScheduler,
    apic: X2Apic,
    timer: ApicTimer,
}

impl LpIsaDataIfce for LpIsaData {
    fn new() -> LpIsaData {
        LpIsaData {
            curr_tcb: None,
            idt: Idt::new(),
            local_scheduler: LocalScheduler::new(Box::new(RoundRobin::new())),
            apic: <X2Apic as LocalIntCtlrIfce>::new(),
            timer: ApicTimer::new(),
        }
    }

    fn get() -> &'static Self {
        let ret: *const Self;
        unsafe {
            core::arch::asm! {
                "mov {}, gs:[0]",
                out(reg) ret
            };
            &*ret
        }
    }

    fn get_mut() -> &'static mut Self {
        let ret: *mut Self;
        unsafe {
            core::arch::asm! {
                "mov {}, gs:[0]",
                out(reg) ret
            };
            &mut *ret
        }
    }

    fn get_lic(&mut self) -> &mut LocalIntCtlr {
        &mut self.apic
    }

    fn get_lp_timer(&mut self) -> &mut LpTimer {
        &mut self.timer
    }
}
