mod simple_rr;

pub use super::thread::{Thread, ThreadId};
use crate::memory::vmem::AddressSpaceId;
pub enum Error {
    RingOutOfMemory,
}

pub type Duration = u64;

pub trait LocalScheduler {
    extern "C" fn get_current_thread(&self) -> ThreadId;
    extern "C" fn get_current_thread_quantum(&self) -> Duration;
    extern "C" fn advance_thread_iter(&self);
    fn assign_thread(&mut self, thread: Thread) -> Result<(), Error>;
    fn is_address_space_active(&self, asid: AddressSpaceId) -> bool;
    fn assigned_thread_count(&self) -> usize;
}
