use alloc::boxed::Box;
use core::mem::MaybeUninit;

use crate::scheduler::local::{LocalScheduler, ThreadId};

/* This structure contains the global variables that are local to the current logical processor */
struct LpLocal {
    local_scheduler: Box<dyn LocalScheduler>,
}

impl LpLocal {
    pub fn new(lsched: Box<dyn LocalScheduler>) -> Self {
        LpLocal {
            local_scheduler: lsched,
        }
    }
}

pub extern "C" fn get_current_tid() -> ThreadId {
    unsafe {
        let lp_local: *const LpLocal;
        core::arch::asm!(
            "mov {}, gs:0",
            out(reg) lp_local,
        );
        (*lp_local).local_scheduler.get_current_thread()
    }
}
