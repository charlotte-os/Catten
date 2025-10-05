use spin::{Lazy, RwLock};

use crate::common::collections::id_table::IdTable;
use crate::cpu::isa::lp::thread_context::ThreadContext;
use crate::cpu::isa::memory::stack::StackBuf;

type ThreadTable = IdTable<ThreadId, Thread>;
pub type ThreadId = usize;

pub static THREAD_TABLE: Lazy<RwLock<ThreadTable>> = Lazy::new(|| RwLock::new(ThreadTable::new()));

pub struct Thread {
    pub state: ThreadContext,
    _stack_buffer: StackBuf,
}
