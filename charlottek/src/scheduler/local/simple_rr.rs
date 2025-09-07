use alloc::collections::VecDeque;
use alloc::vec::Vec;

use super::{Duration, Thread};
use crate::memory::vmem::AddressSpaceId;

struct AsList {
    asid: Option<AddressSpaceId>,
    threads: Vec<Thread>,
    idx: usize,
}

pub struct SimpleRr {
    execution_ring: VecDeque<AsList>,
    total_thread_count: usize,
    epoch_length: Duration,
}
