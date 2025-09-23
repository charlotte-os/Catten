use alloc::boxed::Box;
use core::arch::asm;

use crate::cpu::scheduler::LpScheduler;
use crate::cpu::threads::{Thread, ThreadId};

#[repr(C)]
pub struct LpLocal {
    current_tid: Option<ThreadId>,
    pub local_scheduler: Box<dyn LpScheduler>,
}

impl LpLocal {
    pub fn new(scheduler: Box<dyn LpScheduler>) -> Self {
        LpLocal {
            current_tid: None,
            local_scheduler: scheduler,
        }
    }

    pub fn get() -> *mut Self {
        let lp_local_ptr: *mut LpLocal;
        unsafe {
            asm!(
                "mov {}, gs:0",
                out(reg) lp_local_ptr,
            );
        }
        lp_local_ptr
    }
}
