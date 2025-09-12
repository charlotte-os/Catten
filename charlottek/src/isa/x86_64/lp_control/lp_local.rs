use alloc::boxed::Box;
use core::mem::MaybeUninit;

use crate::scheduler::local::LocalScheduler;

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
