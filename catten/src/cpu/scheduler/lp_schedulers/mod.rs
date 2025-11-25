//! # Logical Processor Local Schedulers
pub mod strategy;

use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;

use hashbrown::HashMap;

use crate::cpu::isa::memory::paging::HwAsid;
use crate::cpu::scheduler::lp_schedulers::strategy::{LocalSchedStratIfce, RoundRobin};
use crate::cpu::scheduler::threads::ThreadId;
use crate::memory::AddressSpaceId;

type RunQueue = VecDeque<(AddressSpaceId, Vec<ThreadId>)>;

pub struct LocalScheduler {
    run_queue: RunQueue,
    strategy: Box<dyn LocalSchedStratIfce>,
    asid_mapping: HashMap<AddressSpaceId, HwAsid>,
}

#[repr(u8)]
pub enum Status {
    QueueFull,
    ThreadNotFound,
    AsNotFound,
}

impl LocalScheduler {
    pub fn new_round_robin() -> LocalScheduler {
        LocalScheduler {
            run_queue: RunQueue::new(),
            strategy: Box::new(RoundRobin::new()),
            asid_mapping: HashMap::new(),
        }
    }

    pub fn add_thread(&mut self, tid: ThreadId) -> Status {
        todo!()
    }

    pub fn remove_threads(&mut self, tids: Vec<ThreadId>) {
        todo!()
    }

    pub fn remove_as(&mut self, asid: AddressSpaceId) {
        todo!()
    }

    pub fn is_idle(&self) -> bool {
        self.run_queue.is_empty()
    }

    pub fn asid_to_hwasid(&self, asid: AddressSpaceId) -> Option<HwAsid> {
        todo!()
    }
}
