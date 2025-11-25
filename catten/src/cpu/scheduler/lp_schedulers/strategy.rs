use super::*;
use crate::memory::AddressSpaceId;

pub unsafe trait LocalSchedStratIfce {
    fn next_thread(&mut self, run_queue: &mut RunQueue) -> Option<ThreadId>;
}

pub struct RoundRobin {
    curr_asid: AddressSpaceId,
    as_vec: Vec<ThreadId>,
    as_thread_idx: usize,
}

impl RoundRobin {
    pub fn new() -> RoundRobin {
        RoundRobin {
            curr_asid: AddressSpaceId::default(),
            as_vec: Vec::new(),
            as_thread_idx: usize::default(),
        }
    }
}

unsafe impl LocalSchedStratIfce for RoundRobin {
    fn next_thread(&mut self, run_queue: &mut RunQueue) -> Option<ThreadId> {
        if self.as_vec.len() == (*self).as_thread_idx + 1 {
            let curr = (self.curr_asid, self.as_vec.clone());
            run_queue.push_back(curr);
            let next = run_queue.pop_front()?;
            self.curr_asid = next.0;
            self.as_vec = next.1;
            self.as_thread_idx = 0;
        } else {
            self.as_thread_idx += 1;
        }

        Some(self.as_vec[self.as_thread_idx])
    }
}
