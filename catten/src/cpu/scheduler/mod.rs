use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;

use spin::Mutex;
use spin::rwlock::RwLock;

use crate::cpu::threads::{Thread, ThreadId};
use crate::isa::lp::ops::get_lp_id;
use crate::memory::AddressSpaceId;

pub static GLOBAL_SCHEDULER: GlobalScheduler = GlobalScheduler::new();

pub struct GlobalScheduler {
    blocked_threads: RwLock<BTreeSet<ThreadId>>,
    ready_unassigned: Mutex<VecDeque<ThreadId>>,
    lp_schedulers: Vec<Mutex<Box<dyn LpScheduler>>>,
}

impl GlobalScheduler {
    pub const fn new() -> Self {
        Self {
            blocked_threads: RwLock::new(BTreeSet::new()),
            ready_unassigned: Mutex::new(VecDeque::new()),
            lp_schedulers: Vec::new(),
        }
    }

    pub fn get_local_lp_scheduler(&self) -> &Mutex<Box<dyn LpScheduler>> {
        &self.lp_schedulers[get_lp_id!() as usize]
    }
}

unsafe impl Sync for GlobalScheduler {}

pub trait LpScheduler {
    extern "C" fn advance(&self);
    fn add_thread(&mut self, thread: Thread);
    fn terminate_threads(&mut self, thread_ids: Vec<ThreadId>);
    fn abort_threads(&mut self, thread_ids: Vec<ThreadId>);
    fn abort_as_threads(&mut self, asid: AddressSpaceId);
    fn is_idle(&self) -> bool;
    fn asid_to_pcid(&self, asid: AddressSpaceId) -> Option<u16>;
}
