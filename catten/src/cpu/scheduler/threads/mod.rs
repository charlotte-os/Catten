use alloc::boxed::Box;
use alloc::vec;

use spin::{Lazy, RwLock};

use crate::common::collections::id_table::IdTable;
use crate::cpu::isa::lp::thread_context::ThreadContext;
use crate::memory::{AddressSpaceId, VAddr};

pub static THREAD_TABLE: Lazy<RwLock<ThreadTable>> = Lazy::new(|| RwLock::new(ThreadTable::new()));

type ThreadTable = IdTable<ThreadId, Thread>;

pub type ThreadId = usize;

pub struct Thread {
    state: ThreadContext,
    _stack_buffer: Box<[u8]>,
}

impl Thread {
    pub fn new(stack_size: usize, asid: AddressSpaceId, entry_point: VAddr) -> Self {
        let mut stack_buffer = vec![0u8; stack_size].into_boxed_slice();
        let stack_top = stack_buffer.as_mut_ptr() as usize + stack_size;
        let state = ThreadContext::new(asid, VAddr::from(stack_top), entry_point);
        Thread {
            state,
            _stack_buffer: stack_buffer,
        }
    }

    pub fn get_state(&self) -> &ThreadContext {
        &self.state
    }
}
