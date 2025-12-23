use alloc::vec::Vec;

use spin::Lazy;

use crate::common::collections::id_table::IdTable;
use crate::cpu::isa::lp::LpId;
use crate::cpu::isa::lp::thread_context::ThreadContext;
use crate::event::Completion;
use crate::memory::{AddressSpaceId, VAddr};

pub static mut MASTER_THREAD_TABLE: Lazy<ThreadTable> = Lazy::new(ThreadTable::new);
pub type ThreadTable = IdTable<ThreadId, Thread>;
pub type ThreadId = usize;

pub enum ThreadState {
    Running(LpId),
    Ready(LpId),
    NeedsLpAssignment,
    Blocked(Vec<Completion>),
    Terminated, //Used while the thread is being cleaned up
}

pub struct Thread {
    pub is_user: bool,
    pub context: ThreadContext,
    pub asid: AddressSpaceId,
    pub state: ThreadState,
}

impl Thread {
    pub fn new(is_user: bool, asid: AddressSpaceId, entry_point: VAddr) -> Self {
        Thread {
            is_user,
            context: ThreadContext::new(asid, entry_point).expect(""),
            asid,
            state: ThreadState::NeedsLpAssignment,
        }
    }
}
