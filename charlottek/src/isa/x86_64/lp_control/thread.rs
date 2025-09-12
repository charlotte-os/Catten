use alloc::collections::VecDeque;
use alloc::vec::Vec;

use spin::RwLock;

use crate::isa::x86_64::lp_control::{LpControl, LpControlIfce};
use crate::memory::pmem::PAddr;

pub type ThreadId = usize;

static mut THREAD_LIST: Vec<Option<RwLock<Thread>>> = Vec::new();

pub enum Error {
    InvalidLp,
}
#[repr(C, packed)]
pub struct Thread {
    cr3: u64,
    rsp: PAddr,
    assigned_lp: Option<<LpControl as LpControlIfce>::LpId>,
    recent_lps: VecDeque<<LpControl as LpControlIfce>::LpId>,
}
