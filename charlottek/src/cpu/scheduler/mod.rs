use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;

use hashbrown::HashSet;
use spin::Mutex;

use crate::cpu::threads::{Thread, ThreadId};
use crate::memory::AddressSpaceId;

pub static GLOBAL_SCHEDULER: spin::Lazy<Mutex<GlobalScheduler>> =
    spin::Lazy::new(|| Mutex::new(GlobalScheduler::new()));

pub struct GlobalScheduler {
    blocked_threads:  HashSet<ThreadId>,
    ready_unassigned: VecDeque<ThreadId>,
}

impl GlobalScheduler {
    pub fn new() -> Self {
        Self {
            blocked_threads:  HashSet::new(),
            ready_unassigned: VecDeque::new(),
        }
    }
}

pub trait LpScheduler {
    extern "C" fn advance(&self);
    fn add_thread(&mut self, thread: Thread);
    fn terminate_threads(&mut self, thread_ids: Vec<ThreadId>);
    fn abort_threads(&mut self, thread_ids: Vec<ThreadId>);
    fn abort_as_threads(&mut self, asid: AddressSpaceId);
    fn is_idle(&self) -> bool;
    fn asid_to_pcid(&self, asid: AddressSpaceId) -> Option<u16>;
}
