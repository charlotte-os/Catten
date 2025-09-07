mod simple_rr;

pub use crate::isa::current_isa::lp_control::thread::Thread;
use crate::memory::vmem::AddressSpaceId;
pub enum Error {
    RingOutOfMemory,
}

pub type Duration = u64;

pub trait LocalScheduler {
    extern "C" fn get_current_thread() -> *mut Thread;
    extern "C" fn get_current_thread_quantum() -> Duration;
    extern "C" fn advance_thread_iter();
    fn assign_thread(&mut self, thread: Thread) -> Result<(), Error>;
    fn is_address_space_active(&self, asid: AddressSpaceId) -> bool;
    fn assigned_thread_count(&self) -> usize;
}
