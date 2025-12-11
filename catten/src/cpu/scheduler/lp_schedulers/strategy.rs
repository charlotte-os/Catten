use super::*;
use crate::memory::AddressSpaceId;

/// Local Scheduling Strategy Interface
pub unsafe trait LsStratIfce {
    fn next_thread(&mut self, run_queue: &mut RunQueue) -> Option<ThreadId>;
    fn next_as(&mut self, run_queue: &mut RunQueue) -> Option<AddressSpaceId>;
    fn get_curr_as(&self) -> AddressSpaceId;
}
/// Simple Round Robin Local Scheduling Strategy
pub struct RoundRobin {
    /// Current Address Space ID
    curr_asid: AddressSpaceId,
    /// Index of the current thread in the Thread Vector
    as_thread_idx: usize,
}

impl RoundRobin {
    /// Constructor
    pub fn new() -> RoundRobin {
        RoundRobin {
            curr_asid: AddressSpaceId::default(),
            as_thread_idx: usize::default(),
        }
    }
}

unsafe impl LsStratIfce for RoundRobin {
    fn next_thread(&mut self, run_queue: &mut RunQueue) -> Option<ThreadId> {
        if run_queue.is_empty() {
            None
        } else {
            if let Some(as_vec) = run_queue.get(&self.curr_asid) {
                if as_vec.len() == (*self).as_thread_idx + 1 {
                    let mut key_iter = run_queue.keys();
                    while key_iter.next() != Some(&self.curr_asid) {}
                    self.curr_asid = match key_iter.next() {
                        Some(asid) => *asid,
                        None => *run_queue.keys().nth(0).unwrap(),
                    };
                    self.as_thread_idx = 0;
                } else {
                    self.as_thread_idx += 1;
                }
            }

            Some(run_queue.get(&self.curr_asid).unwrap()[self.as_thread_idx])
        }
    }

    fn next_as(&mut self, run_queue: &mut RunQueue) -> Option<AddressSpaceId> {
        if run_queue.is_empty() {
            None
        } else {
            let mut key_iter = run_queue.keys();
            while key_iter.next() != Some(&self.curr_asid) {}
            self.curr_asid = match key_iter.next() {
                Some(asid) => *asid,
                None => *run_queue.keys().nth(0).unwrap(),
            };
            self.as_thread_idx = 0;
            Some(self.curr_asid)
        }
    }

    fn get_curr_as(&self) -> AddressSpaceId {
        self.curr_asid
    }
}
