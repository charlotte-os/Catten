//! # Inter-Processor Interrupts (IPIs) on the x86_64 Architecture
//!
//! The charlottek IPI protocol is designed to work using remote procedure calls (RPCs).
//! This allows for a flexible and extensible way to send IPIs between processors.
//! The protocol supports both unicast (single target) and multicast (multiple targets) IPIs.
//! The implementation is kept as similar as possible across different architectures within reason.
//!
//! Each logical processor (LP) has it's own IPI mailbox, which is contains an enum with the IPI
//! type and arguments. Sending an IPI involves writing to the target LP's mailbox and then
//! triggering the IPI via the architecture-specific mechanism. Receiving an IPI involves checking
//! the mailbox and executing the corresponding handler. If the IPI is multicast, the first argument
//! passed is a pointer to the completion barrier, which is used to signal when all target LPs have
//! completed handling the IPI. This is important for ensuring that all target LPs have completed
//! the requested operation before any of them return from the ISR.

use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;
use core::arch::global_asm;

use spin::Mutex;

use crate::cpu::scheduler::GLOBAL_SCHEDULER;
use crate::cpu::threads::ThreadId;
use crate::isa::memory::tlb;
use crate::memory::vmem::VAddr;
use crate::memory::{AddressSpaceId, KERNEL_ASID};

#[unsafe(no_mangle)]
pub static GS_OFFSET_IPI_QUEUE: usize = 16;

global_asm!(include_str!("ipis.asm"));

unsafe extern "C" {
    pub fn isr_interprocessor_interrupt();
}

#[derive(Clone, Debug)]
pub enum Ipi {
    VMemInval(AddressSpaceId, VAddr, usize),
    AsidInval(AddressSpaceId),
    TerminateThreads(Vec<ThreadId>),
    AbortThreads(Vec<ThreadId>),
    AbortAsThreads(AddressSpaceId),
}

#[unsafe(no_mangle)]
pub extern "C" fn ih_interprocessor_interrupt(ipi_queue: &'static mut Mutex<VecDeque<Ipi>>) {
    while let Some(ipi) = ipi_queue.lock().pop_front() {
        match ipi {
            Ipi::VMemInval(asid, base, size) => {
                if asid == KERNEL_ASID {
                    tlb::inval_range_kernel(base, size);
                } else {
                    tlb::inval_range_user(asid, base, size);
                }
            }
            Ipi::AsidInval(asid) => tlb::inval_asid(asid),
            Ipi::TerminateThreads(tids) => {
                GLOBAL_SCHEDULER.get_local_lp_scheduler().lock().terminate_threads(tids)
            }
            Ipi::AbortThreads(tids) => {
                GLOBAL_SCHEDULER.get_local_lp_scheduler().lock().abort_threads(tids)
            }
            Ipi::AbortAsThreads(asid) => {
                GLOBAL_SCHEDULER.get_local_lp_scheduler().lock().abort_as_threads(asid)
            }
        }
    }
}
