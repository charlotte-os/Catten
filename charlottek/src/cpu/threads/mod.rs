use spin::{Lazy, RwLock};

use crate::common::collections::id_table::IdTable;
use crate::cpu::isa::lp::thread_context::ThreadContext;
use crate::cpu::isa::lp::{LogicalProcessor, LpIfce};
use crate::event::Completion;
use crate::memory::stack::{Error as StackError, StackBuf};

type ThreadTable = IdTable<ThreadId, Thread>;
pub type ThreadId = usize;
pub type ThreadExitCode = isize;

pub static THREAD_TABLE: Lazy<RwLock<ThreadTable>> = Lazy::new(|| RwLock::new(ThreadTable::new()));

pub enum Error {
    StackError(StackError),
}

impl From<StackError> for Error {
    fn from(se: StackError) -> Error {
        Error::StackError(se)
    }
}

pub enum ThreadState {
    RunnableUnassigned,
    RunnableQueued(<LogicalProcessor as LpIfce>::LpId),
    Running(<LogicalProcessor as LpIfce>::LpId),
    Blocked(Completion<fn()>),
    Terminated(ThreadExitCode),
}

pub struct Thread {
    context: ThreadContext,
    _stack_buffer: StackBuf,
    state: ThreadState,
}

impl Thread {
    pub fn new_runnable(stack_size: usize) -> Result<Self, Error> {
        Ok(Thread {
            context: ThreadContext::default(),
            _stack_buffer: StackBuf::try_new(stack_size)?,
            state: ThreadState::RunnableUnassigned,
        })
    }
}
