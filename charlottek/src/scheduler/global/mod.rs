use alloc::sync::Arc;
use core::ops::Add;

use spin::RwLock;

use super::thread::ThreadId;
use crate::event::{Completion, Event};
use crate::isa::interface::memory::address::Address;
use crate::isa::x86_64::lp_control::{LpControl, LpControlIfce};
use crate::memory::vmem::AddressSpaceId;

pub enum Error {
    InvalidThread,
}

pub enum ThreadState {
    Active(<LpControl as LpControlIfce>::LpId),
    Running(<LpControl as LpControlIfce>::LpId),
    Blocked,
    Terminated,
}

pub trait GlobalScheduler {
    type Stats: core::fmt::Debug + core::fmt::Display;

    fn activate_thread(&mut self, thread: ThreadId);
    fn block_thread(&mut self, thread: ThreadId, blocker: Completion);
    fn terminate_thread(&mut self, thread: ThreadId);
    fn get_stats(&self) -> Self::Stats;
    fn get_thread_state(&self, thread: ThreadId) -> ThreadState;
    /*WARNING: This function sends a broadcast IPI that stops the world and
     * reassigns all threads. This is obstructive and can cause short term performance
     * degradation. Do not use it unless absolutely necessary. The scheduler
     * implementations already opportunistically load balance when unblocking threads.
     */
    fn force_load_balance(&mut self);
    /* Terminates all threads in the specified address space
     * Useful for OSes using this kernel that have a concept of processes with multiple threads
     * despite the kernel itself having no such concept.
     */
    fn terminate_all_as(&mut self, asid: AddressSpaceId);
}
