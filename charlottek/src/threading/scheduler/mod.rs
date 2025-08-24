use crate::event::Completion;
use crate::isa::current_isa::lp_control::{LpControl, LpControlIfce};
use crate::threading::thread::ThreadGuid;

pub trait Scheduler {
    extern "C" fn get_current_thread(lp: <LpControl as LpControlIfce>::LpId) -> ThreadGuid;
    extern "C" fn get_next_thread(lp: <LpControl as LpControlIfce>::LpId) -> ThreadGuid;
    extern "C" fn block_thread(thread: ThreadGuid, until: Completion);
}
