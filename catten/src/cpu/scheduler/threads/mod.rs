use spin::Lazy;

use crate::common::collections::id_table::IdTable;
use crate::cpu::isa::lp::LpId;
use crate::cpu::isa::lp::thread_context::ThreadContext;
use crate::memory::{AddressSpaceId, VAddr};

static mut THREAD_TABLE: Lazy<ThreadTable> = Lazy::new(ThreadTable::new);
type ThreadTable = IdTable<ThreadId, Thread>;

const LP_AFFINITY_COUNT: usize = 8;

pub type ThreadId = usize;

pub struct Thread {
    is_user: bool,
    context: ThreadContext,
    asid: AddressSpaceId,
    lp_affinity: [LpId; LP_AFFINITY_COUNT],
}

impl Thread {
    pub fn new(is_user: bool, asid: AddressSpaceId, entry_point: VAddr) -> Self {
        Thread {
            is_user,
            context: ThreadContext::new(asid, entry_point).expect(""),
            asid,
            lp_affinity: [LpId::default(); LP_AFFINITY_COUNT],
        }
    }
}
