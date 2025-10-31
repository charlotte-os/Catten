//! # Logical Processor Local Schedulers
use alloc::collections::btree_map::BTreeMap;
use alloc::collections::btree_set::BTreeSet;

use crate::cpu::isa::memory::paging::HwAsid;
use crate::cpu::scheduler::threads::{THREAD_TABLE, ThreadId};
use crate::memory::AddressSpaceId;

pub struct LocalScheduler {
    run_queue: BTreeMap<AddressSpaceId, BTreeSet<ThreadId>>,
    index: (AddressSpaceId, usize),
    advance: extern "C" fn(&mut Self) -> Status,
}

#[repr(u8)]
pub enum Status {
    Success,
    Idle,
    QueueFull,
    ThreadNotFound,
    AsNotFound,
}

impl LocalScheduler {
    pub fn new() -> Self {
        LocalScheduler {
            run_queue: BTreeMap::new(),
            index: (0, 0),
            advance: LocalScheduler::default_advance,
        }
    }

    extern "C" fn default_advance(scheduler: &mut LocalScheduler) -> Status {
        if scheduler.is_idle() {
            Status::Idle
        } else {
            if scheduler.index.1 + 1 < scheduler.run_queue.len() {
                scheduler.index.1 += 1;
            } else {
                scheduler.index.0 =
                    (scheduler.index.0 + 1) % scheduler.run_queue.len() as AddressSpaceId;
                scheduler.index.1 = 0;
            }
            Status::Success
        }
    }

    pub fn add_thread(&mut self, tid: ThreadId) -> Status {
        if let Some(thread) = THREAD_TABLE.read().try_get_element_arc(tid) {
            let asid = (*thread).read().get_state().get_asid();
            if self.run_queue.contains_key(&asid) {
                self.run_queue.get_mut(&asid).unwrap().insert(tid);
            } else {
                let mut thread_set = BTreeSet::new();
                thread_set.insert(tid);
                self.run_queue.insert(asid, thread_set);
            }
            Status::Success
        } else {
            Status::ThreadNotFound
        }
    }

    pub fn remove_threads(&mut self, thread_ids: BTreeSet<ThreadId>) {
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
