pub mod lp_local;

use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;

use hashbrown::HashSet;
use lp_local::LpScheduler;
use spin::{Lazy, Mutex, RwLock};

use crate::cpu::isa::lp::{LogicalProcessor, LpIfce};
use crate::cpu::threads::ThreadId;

pub static GLOBAL_SCHEDULER: Lazy<GlobalScheduler> = Lazy::new(GlobalScheduler::new);

pub struct GlobalScheduler {
    blocked_threads: RwLock<HashSet<ThreadId>>,
    ready_unassigned: Mutex<VecDeque<ThreadId>>,
    lp_schedulers: Vec<Mutex<Box<dyn LpScheduler>>>,
}

impl GlobalScheduler {
    pub fn new() -> Self {
        Self {
            blocked_threads: RwLock::new(HashSet::new()),
            ready_unassigned: Mutex::new(VecDeque::new()),
            lp_schedulers: Vec::new(),
        }
    }

    pub fn get_local_lp_scheduler(&self) -> &Mutex<Box<dyn LpScheduler>> {
        &self.lp_schedulers[LogicalProcessor::read_lp_id() as usize]
    }

    pub fn unblock_thread(&self, tid: ThreadId) {
        self.blocked_threads.write().remove(&tid);
        self.ready_unassigned.lock().push_back(tid);
    }
}

unsafe impl Send for GlobalScheduler {}
