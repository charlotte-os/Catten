//! # Inter-Processor Interrupts (IPIs)
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

use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;
use core::alloc::Layout;
use core::ptr::slice_from_raw_parts_mut;
use core::sync::atomic::Ordering::SeqCst;
use core::sync::atomic::{AtomicPtr, AtomicU32};

use spin::{Lazy, Mutex};

use crate::cpu::isa::lp::LpId;
use crate::cpu::isa::memory::tlb;
use crate::cpu::scheduler::system_scheduler::SYSTEM_SCHEDULER;
use crate::cpu::scheduler::threads::ThreadId;
use crate::memory::linear::VAddr;
use crate::memory::{AddressSpaceId, KERNEL_ASID};

pub struct IpiRpcReq {
    pub sender_lp_id: LpId,
    pub recipient_lp_ids: Vec<LpId>,
    pub request_id: u64,
    pub rpc: IpiRpc,
    pub hash: u64,
    completion_barrier: AtomicU32,
}

pub static IPI_RPC_MAILBOXES: Lazy<Box<[IpiRpcMailbox]>> = Lazy::new(|| {
    /* Getting a Boxed slice of a type that isn't Clone and whose count you only know at runtime
     * is a major pain in the ass.
     */

    let num_lps = crate::cpu::multiprocessor::get_lp_count() as usize;
    unsafe {
        let ptr = alloc::alloc::alloc_zeroed(Layout::from_size_align_unchecked(
            size_of::<IpiRpcMailbox>() * num_lps,
            align_of::<IpiRpcMailbox>(),
        ));
        Box::from_raw(slice_from_raw_parts_mut(ptr as *mut IpiRpcMailbox, num_lps))
    }
});

pub struct IpiRpcMailbox {
    req: AtomicPtr<IpiRpcReq>,
}

impl IpiRpcMailbox {
    pub const fn new() -> Self {
        Self {
            req: AtomicPtr::new(core::ptr::null_mut()),
        }
    }

    pub fn try_write(&self, req: *mut IpiRpcReq) -> Result<*mut IpiRpcReq, *mut IpiRpcReq> {
        self.req.compare_exchange(core::ptr::null_mut(), req, SeqCst, SeqCst)
    }

    pub fn read(&self) -> *mut IpiRpcReq {
        self.req.load(SeqCst)
    }
}

unsafe impl Send for IpiRpcMailbox {}
unsafe impl Sync for IpiRpcMailbox {}

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
            IpiRpc::TerminateThreads(tids) => SYSTEM_SCHEDULER.terminate_threads(tids),
            IpiRpc::AbortThreads(tids) => SYSTEM_SCHEDULER.abort_threads(tids),
            IpiRpc::AbortAsThreads(asid) => SYSTEM_SCHEDULER.abort_as_threads(asid),
        }
    }
}
