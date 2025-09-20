use alloc::boxed::Box;

use crate::isa::target::lp::{CoreState, LpId};

pub type ThreadId = usize;

pub struct Thread {
    pub id: ThreadId,
    // Additional fields can be added here
    state: CoreState,
    stack_buffer: Box<[u8]>,
}

pub trait Scheduler {
    type Config;

    fn current_thread(&self) -> ThreadId;
    fn next_thread(&self) -> Option<ThreadId>;
    fn add_thread(&mut self, thread: Thread);
    fn terminate_thread(&mut self, thread_id: ThreadId);
    fn abort_thread(&mut self, thread_id: ThreadId);
    fn get_config(&self) -> &Self::Config;
    fn set_config(&mut self, config: Self::Config);
}
