//! # Logical Processor Local Schedulers
use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;

use crate::cpu::isa::memory::paging::HwAsid;
use crate::cpu::scheduler::threads::ThreadId;
use crate::memory::AddressSpaceId;

pub struct LocalScheduler {
    run_queue: BTreeMap<AddressSpaceId, Vec<ThreadId>>,
    index: (AddressSpaceId, usize),
    advance: extern "C" fn(&mut Self) -> Status,
}

#[repr(u8)]
pub enum Status {
    QueueFull,
    ThreadNotFound,
    AsNotFound,
}

impl LocalScheduler {
    pub fn add_thread(&mut self, thread: ThreadId) -> Status {
        todo!()
    }

    pub fn remove_threads(&mut self, thread_ids: Vec<ThreadId>) {
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
