//! # Inter-Processor Interrupts (IPIs) on the x86_64 Architecture
//!
//! The Catten IPI protocol is designed to work using remote procedure calls (RPCs).
//! This allows for a flexible and extensible way to send IPIs between processors.
//! The protocol supports both unicast (single target) and multicast (multiple targets) IPIs.
//! The implementation is kept as similar as possible across different architectures within reason.
//!
//! Each logical processor (LP) has its own IPI mailbox, which holds a pointer to the the requested
//! RPC and its arguments. To send an IPI create the RPC type instance and attempt to write its
//! address into each target LP's mailbox but only if it is currenlty null using an atomic
//! compare-and-swap (CAS) operation. If you fail to write even one of the the target LPs'
//! mailboxes, revert the ones you did write back to null. When writing to multiple mailboxes you
//! MUST do so in order of ascending LP ID to avoid deadlocks. If you are able to write to all
//! target mailboxes, then immediately send an IPI interrupt to each target LP to trigger the IPI
//! ISR and have your RPC be executed. At the end of the IPI ISR the target LP MUST set its
//! mailbox back to null to indicate it is ready to receive another IPI.

use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;
use core::arch::global_asm;

use spin::Mutex;

use crate::cpu::isa::memory::tlb;
use crate::cpu::scheduler::system_scheduler::SYSTEM_SCHEDULER;
use crate::cpu::scheduler::threads::ThreadId;
use crate::memory::linear::VAddr;
use crate::memory::{AddressSpaceId, KERNEL_ASID};

#[unsafe(no_mangle)]
pub static GS_OFFSET_IPI_MAILBOX: usize = 16;

global_asm!(include_str!("ipis.asm"));

unsafe extern "C" {
    pub fn isr_interprocessor_interrupt();
}

#[derive(Clone, Debug)]
pub enum IpiRpc {
    VMemInval(AddressSpaceId, VAddr, usize),
    AsidInval(AddressSpaceId),
    TerminateThreads(Vec<ThreadId>),
    AbortThreads(Vec<ThreadId>),
    AbortAsThreads(AddressSpaceId),
}

#[unsafe(no_mangle)]
pub extern "C" fn ih_interprocessor_interrupt(ipi_queue: &'static mut Mutex<VecDeque<IpiRpc>>) {
    while let Some(ipi) = ipi_queue.lock().pop_front() {
        match ipi {
            IpiRpc::VMemInval(asid, base, size) => {
                if asid == KERNEL_ASID {
                    tlb::inval_range_kernel(base, size);
                } else {
                    tlb::inval_range_user(asid, base, size);
                }
            }
            IpiRpc::AsidInval(asid) => tlb::inval_asid(asid),
            IpiRpc::TerminateThreads(tids) => {
                SYSTEM_SCHEDULER.get_local_scheduler().lock().terminate_threads(tids)
            }
            IpiRpc::AbortThreads(tids) => {
                SYSTEM_SCHEDULER.get_local_scheduler().lock().abort_threads(tids)
            }
            IpiRpc::AbortAsThreads(asid) => {
                SYSTEM_SCHEDULER.get_local_scheduler().lock().abort_as_threads(asid)
            }
        }
    }
}
