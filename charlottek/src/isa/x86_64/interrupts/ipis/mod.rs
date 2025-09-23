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

use alloc::vec::Vec;

use crate::cpu::threads::ThreadId;
use crate::isa::lp::lp_local::LpLocal;
use crate::isa::memory::tlb;
use crate::memory::vmem::VAddr;
use crate::memory::{AddressSpaceId, KERNEL_ASID};

enum Ipi {
    VMemInval(AddressSpaceId, VAddr, usize),
    AsidInval(AddressSpaceId),
    TerminateThreads(Vec<ThreadId>),
    AbortThreads(Vec<ThreadId>),
    AbortAsThreads(AddressSpaceId),
}

#[unsafe(no_mangle)]
pub extern "C" fn ih_ipi(mailbox: &'static mut Option<Ipi>) {
    if let Some(ipi) = mailbox.take() {
        match ipi {
            Ipi::VMemInval(asid, base, size) => {
                if asid == KERNEL_ASID {
                    tlb::inval_range_kernel(base, size);
                } else {
                    // SAFETY: This is safe because we are executing in an interrupt context where
                    // preemption is disabled, and we are not modifying any data structures that
                    // could be accessed by other threads.
                    tlb::inval_range_user(asid, base, size);
                }
            }
            Ipi::AsidInval(asid) => tlb::inval_asid(asid),
            Ipi::TerminateThreads(tids) => unsafe {
                (*LpLocal::get()).local_scheduler.terminate_threads(tids)
            },
            Ipi::AbortThreads(tids) => unsafe {
                (*LpLocal::get()).local_scheduler.abort_threads(tids)
            },
            Ipi::AbortAsThreads(asid) => unsafe {
                (*LpLocal::get()).local_scheduler.abort_as_threads(asid)
            },
        }
    }
}
