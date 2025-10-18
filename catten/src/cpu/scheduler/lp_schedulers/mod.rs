//! # Logical Processor Local Schedulers
use alloc::vec::Vec;

use crate::cpu::scheduler::threads::{Thread, ThreadId};
use crate::memory::AddressSpaceId;

/// Dyn compatible trait for logical processor local schedulers
pub trait LpLocalScheduler {
    extern "C" fn advance(&self);
    fn add_thread(&mut self, thread: Thread);
    fn terminate_threads(&mut self, thread_ids: Vec<ThreadId>);
    fn abort_threads(&mut self, thread_ids: Vec<ThreadId>);
    fn abort_as_threads(&mut self, asid: AddressSpaceId);
    fn is_idle(&self) -> bool;
    fn asid_to_pcid(&self, asid: AddressSpaceId) -> Option<u16>;
}
