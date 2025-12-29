use alloc::collections::btree_map::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;

use spin::Mutex;

use super::lp_schedulers::LocalScheduler;
use crate::cpu::isa::lp::LpId;
use crate::cpu::isa::lp::ops::get_lp_id;
use crate::cpu::scheduler::threads::ThreadId;
use crate::event::Event;
use crate::memory::AddressSpaceId;

pub static SYSTEM_SCHEDULER: SystemScheduler = SystemScheduler::new();

pub enum Error {
    InvalidThread,
}

/// The system-wide thread scheduler
pub struct SystemScheduler {
    lp_schedulers: BTreeMap<LpId, Arc<Mutex<LocalScheduler>>>,
}

impl SystemScheduler {
    pub const fn new() -> Self {
        Self {
            lp_schedulers: BTreeMap::new(),
        }
    }

    pub fn get_local_scheduler(&self) -> Arc<Mutex<LocalScheduler>> {
        self.lp_schedulers[&get_lp_id()].clone()
    }

    pub fn submit_ready_thread(&self, tid: ThreadId) -> Result<LpId, Error> {
        todo!()
    }

    /// Yield the current LP's execution to the scheduler
    /// This differs from blocking in that the processor state on entry is discarded
    pub unsafe fn yield_lp(&self) -> ! {
        todo!()
    }

    /// Block the specified thread at least until the given event notifies its observers
    pub fn block_tid(&self, tid: ThreadId, event: &dyn Event) -> Result<(), Error> {
        /* Crate a completion object registered with event and push it to the back of the blocker
        queue for the specified thread. If the tid doesn't point to any thread structure then
        return Error::InvalidThread. If the thread is not already blocked then send a broadcast
        over the kernel IPI-RPC protocol with the EvictThread command. */
        todo!()
    }

    pub fn terminate_threads(&self, tids: Vec<ThreadId>) {
        todo!()
    }

    pub fn abort_threads(&self, tids: Vec<ThreadId>) {
        todo!()
    }

    pub fn abort_as_threads(&self, asid: AddressSpaceId) {
        todo!()
    }
}

unsafe impl Sync for SystemScheduler {}
