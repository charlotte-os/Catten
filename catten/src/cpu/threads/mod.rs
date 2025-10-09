use alloc::boxed::Box;
use alloc::vec::Vec;

use hashbrown::HashMap;
use spin::{Lazy, Mutex, RwLock, RwLockReadGuard};

use crate::common::collections::id_table::IdTable;
use crate::cpu::isa::lp::thread_context::ThreadContext;

static mut THREAD_TABLE: Lazy<ThreadTable> = Lazy::new(ThreadTable::new);

type ThreadTable = IdTable<ThreadId, Thread>;

pub type ThreadId = usize;

pub struct Thread {
    state: ThreadContext,
    stack_buffer: Box<[u8]>,
}
