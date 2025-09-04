use alloc::sync::Arc;
use alloc::vec::Vec;

use spin::RwLock;

use crate::isa::x86_64::lp_control::{LpControl, LpControlIfce};
use crate::memory::pmem::PAddr;
use crate::process_mgmt::ProcessId;

type ThreadId = u64;

static THREAD_LIST: RwLock<Vec<Option<Arc<RwLock<Thread>>>>> = RwLock::new(Vec::new());

pub enum Error {
    InvalidLp,
}
#[repr(C, packed)]
pub struct Thread {
    cr3: u64,
    rsp: PAddr,
    assigned_lp: Option<<LpControl as LpControlIfce>::LpId>,
    lp_affinity: Vec<u8>,
    lp_whitelist: Vec<u8>,
}
