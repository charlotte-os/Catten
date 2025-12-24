use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use alloc::format;
use alloc::sync::Arc;

use spin::mutex::Mutex;

use crate::cpu::isa::interface::memory::address::VirtualAddress;
use crate::cpu::isa::lp::LpId;
use crate::cpu::isa::lp::ops::set_lp_local_base;
use crate::cpu::scheduler::threads::ThreadId;
use crate::memory::{AddressSpaceId, VAddr};

const AS_AFFINITY_COUNT: usize = 4095;

pub struct LogicalProcessor {
    pub id: LpId,
    pub as_affinities: [Option<AddressSpaceId>; AS_AFFINITY_COUNT],
    pub exec_queue_ptr: Arc<Mutex<VecDeque<ThreadId>>>,
}

impl LogicalProcessor {
    pub fn setup(id: LpId, exec_queue_ptr: Arc<Mutex<VecDeque<ThreadId>>>) {
        let lp_struct = Box::try_new(LogicalProcessor {
            id,
            as_affinities: [None; AS_AFFINITY_COUNT],
            exec_queue_ptr,
        })
        .expect(&format!(
            "Failed to allocate an LP struct for LP{}. Main memory is insufficient for core \
             kernel functionality.",
            id
        ));
        let vaddr = VAddr::from_mut(Box::into_raw(lp_struct));
        set_lp_local_base(vaddr);
    }
}
