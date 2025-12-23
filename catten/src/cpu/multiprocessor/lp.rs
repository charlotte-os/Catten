use alloc::collections::vec_deque::VecDeque;

use crate::cpu::scheduler::threads::ThreadId;
use crate::memory::AddressSpaceId;

const AS_AFFINITY_COUNT: usize = 4095;

pub struct LogicalProcessor {
    pub as_affinities:  [Option<AddressSpaceId>; AS_AFFINITY_COUNT],
    pub exec_queue_ptr: *mut VecDeque<ThreadId>,
}
