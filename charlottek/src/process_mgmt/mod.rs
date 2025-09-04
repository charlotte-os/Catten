use alloc::vec::Vec;

use spin::rwlock::RwLock;

use crate::isa::current_isa::lp_control::thread::Thread;
use crate::isa::current_isa::memory::{MemoryInterface, MemoryInterfaceImpl};

pub static PROCESS_TABLE: RwLock<Vec<RwLock<Process>>> = RwLock::new(Vec::new());

pub type ProcessId = usize;

/* The kernel is treated as a pseudo-process for system management. */
const KERNEL_PID: ProcessId = 0;

pub struct Process {
    id: ProcessId,
    address_space: <MemoryInterfaceImpl as MemoryInterface>::AddressSpace,
    threads: Vec<Thread>,
}
