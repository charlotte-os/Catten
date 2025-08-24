use alloc::boxed::Box;

use crate::memory::vmem::VAddr;
use crate::process_mgmt::ProcessId;

pub type ThreadId = usize;

#[repr(C, packed)]
pub struct ThreadGuid {
    pub process_id: ProcessId,
    pub thread_id:  ThreadId,
}

#[repr(C, packed)]
pub struct Thread {
    pub id: ThreadId,
    pub stack: Box<[u8]>,
    pub stack_pointer: VAddr,
    pub pid: ProcessId,
}

impl Thread {
    pub fn new(id: ThreadId, stack: Box<[u8]>, stack_pointer: VAddr, pid: ProcessId) -> Self {
        Thread {
            id,
            stack,
            stack_pointer,
            pid,
        }
    }
}
