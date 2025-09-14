use alloc::collections::VecDeque;
use alloc::vec::Vec;

use spin::RwLock;

use crate::isa::current_isa::lp_control::core_state::{CoreState, CoreStateIfce};
use crate::isa::x86_64::lp_control::{LpControl, LpControlIfce};

static mut THREAD_TABLE: Vec<RwLock<Thread>> = Vec::new();

pub type ThreadId = usize;

#[repr(i32)]
pub enum Status {
    Success = 0,
    InvalidLp = -1,
    InvalidThread = -2,
}

pub struct Thread {
    core_state: CoreState,
    state: ThreadState,
    recent_lps: VecDeque<<LpControl as LpControlIfce>::LpId>,
}

pub enum ThreadState {
    Active(<LpControl as LpControlIfce>::LpId),
    Running(<LpControl as LpControlIfce>::LpId),
    Blocked,
    Terminated,
}

pub extern "C" fn save_core_state(tid: ThreadId) -> Status {
    if let Some(lock) = unsafe { THREAD_TABLE.get(tid) } {
        let mut thread = lock.write();
        thread.core_state.save();
        Status::Success
    } else {
        Status::InvalidThread
    }
}

pub extern "C" fn load_core_state(tid: ThreadId) -> Status {
    if let Some(lock) = unsafe { THREAD_TABLE.get(tid) } {
        let thread = lock.read();
        thread.core_state.load();
        Status::Success
    } else {
        Status::InvalidThread
    }
}
