//! # Logical Processor Local Schedulers
pub mod strategy;

use alloc::boxed::Box;
use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;

use hashbrown::HashMap;

use crate::cpu::isa::lp::ops::halt;
use crate::cpu::isa::memory::paging::HwAsid;
use crate::cpu::scheduler::lp_schedulers::strategy::LsStratIfce;
use crate::cpu::scheduler::threads::{MASTER_THREAD_TABLE, ThreadId};
use crate::memory::AddressSpaceId;

type RunQueue = BTreeMap<AddressSpaceId, Vec<ThreadId>>;

pub struct LocalScheduler {
    run_queue: RunQueue,
    strategy: Box<dyn LsStratIfce>,
    asid_mapping: HashMap<AddressSpaceId, HwAsid>,
}

#[repr(u8)]
pub enum Status {
    Success = 0,
    QueueFull,
    ThreadNotFound,
    AsNotFound,
}

impl LocalScheduler {
    pub fn new(strategy: Box<dyn LsStratIfce>) -> LocalScheduler {
        LocalScheduler {
            run_queue: RunQueue::new(),
            strategy,
            asid_mapping: HashMap::new(),
        }
    }

    pub fn next(&mut self) -> ThreadId {
        if let Some(tid) = self.strategy.next_thread(&mut self.run_queue) {
            tid
        } else {
            // The calling LP is halted and will continue execution when it recieves an interrupt
            // Threads are expected to be added to its local scheduler by the global scheduler
            // before sending it a unicast IPI with the `Wakeup` command.
            halt!()
        }
    }

    pub fn add_thread(&mut self, tid: ThreadId) -> Status {
        if let Some(thread_ptr) = unsafe { MASTER_THREAD_TABLE.try_get_element_arc(tid) } {
            let asid = thread_ptr.read().asid;
            if let Some(as_threads) = self.run_queue.get_mut(&asid) {
                as_threads.push(tid);
                Status::Success
            } else {
                self.run_queue.insert(asid, alloc::vec![tid]);
                Status::Success
            }
        } else {
            Status::ThreadNotFound
        }
    }

    pub fn remove_threads(&mut self, tids: Vec<ThreadId>) {
        todo!()
    }

    pub fn remove_as(&mut self, asid: AddressSpaceId) {
        if self.strategy.get_curr_as() == asid {
            self.strategy.next_as(&mut self.run_queue);
        }
        self.run_queue.remove(&asid);
    }

    pub fn is_idle(&self) -> bool {
        self.run_queue.is_empty()
    }

    pub fn asid_to_hwasid(&self, asid: AddressSpaceId) -> Option<HwAsid> {
        self.asid_mapping.get(&asid).cloned()
    }
}
