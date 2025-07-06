use alloc::boxed::Box;
use alloc::collections::VecDeque;

use spin::Mutex;

use crate::isa::interface::lp_control::LpControlIfce;
use crate::isa::current_isa::lp_control::LpControl;

use super::mp::get_lp_count;

pub type ThreadId = u64;
pub type LpState = <LpControl as LpControlIfce>::LpState;

static THREAD_TABLE: VecDeque<Mutex<Thread>> = VecDeque::new();

#[repr(C, packed)]
pub struct Thread {
    pub id: ThreadId,
    lp_context: LpState,
    flags: u64,
    affinity_mask: VecDeque<u8>,
}