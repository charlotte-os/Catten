use alloc::sync::Arc;
use alloc::vec::Vec;

use hashbrown::HashMap;
use spin::{Lazy, RwLock};

use crate::isa::x86_64::lp_control::{LpControl, LpControlIfce};
use crate::memory::pmem::PAddr;

type ThreadId = u64;

static mut THREAD_LIST: Lazy<HashMap<ThreadId, RwLock<Thread>>> = Lazy::new(HashMap::new);

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
