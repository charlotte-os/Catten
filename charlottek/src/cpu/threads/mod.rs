use alloc::boxed::Box;
use alloc::vec::Vec;

use hashbrown::HashMap;
use spin::{Lazy, Mutex, RwLock, RwLockReadGuard};

use crate::isa::lp::thread_state::ThreadState;
use crate::klib::collections::id_table::IdTable;

static mut ThreadTable: Lazy<Mutex<ThreadTable>> = Lazy::new(|| Mutex::new(ThreadTable::new()));

type ThreadTable = IdTable<ThreadId, Thread>;

pub type ThreadId = usize;

pub struct Thread {
    state: ThreadState,
    stack_buffer: Box<[u8]>,
}
