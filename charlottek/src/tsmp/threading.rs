use alloc::boxed::Box;
use alloc::collections::btree_map::BTreeMap;
use alloc::collections::VecDeque;

use spin::Mutex;

use crate::isa::interface::lp_control::LpControlIfce;
use crate::isa::current_isa::lp_control::LpControl;

use super::mp::get_lp_count;

pub type ThreadId = u64;
pub type LpState = <LpControl as LpControlIfce>::LpState;
pub type LpId = <LpControl as LpControlIfce>::LpId;

static THREAD_TABLE: BTreeMap<LpId, Mutex<Option<Thread>>> = BTreeMap::new();

pub enum ThreadingError {
    LpDoesNotExist,
}

pub fn get_current_tid() -> Result<Option<ThreadId>, ThreadingError> {
    if let Some(thread_lock) = THREAD_TABLE.get(&LpControl::get_lp_id()) {
        let thread = thread_lock.lock();
        match thread.as_ref() {
            Some(t) => Ok(Some(t.id)),
            None => Ok(None),
        }
    } else {
        Err(ThreadingError::LpDoesNotExist)
    }
}

#[repr(C, packed)]
pub struct Thread {
    pub id: ThreadId,
    lp_context: LpState,
    flags: u64,
    affinity_mask: VecDeque<u8>,
}