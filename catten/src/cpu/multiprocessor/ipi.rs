//! # Inter-Processor Interrupts (IPIs)
//!
//! The Catten IPI protocol is designed to work using remote procedure calls (RPCs).
//! This allows for a flexible and extensible way to send IPIs between processors.
//! The protocol supports both unicast (single target), multicast (multiple targets), and broadcast
//! IPIs. The implementation is kept as architecture indepent as possible.

use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering::*;

use spin::barrier::Barrier;
use spin::rwlock::RwLock;
use spin::{Lazy, Mutex};

use crate::common::collections::boxed_slice::make_boxed_slice;
use crate::cpu::isa::lp::LpId;
use crate::cpu::isa::memory::tlb;
use crate::cpu::multiprocessor::get_lp_count;
use crate::cpu::scheduler::system_scheduler::SYSTEM_SCHEDULER;
use crate::cpu::scheduler::threads::ThreadId;
use crate::get_lp_id;
use crate::memory::linear::VAddr;
use crate::memory::{AddressSpaceId, KERNEL_ASID};

pub struct IpiRpcReq {
    pub sender_lp_id: LpId,
    pub recipient_lp_ids: Vec<LpId>,
    pub request_id: u64,
    pub rpc: IpiRpc,
    pub hash: u64,
    pub completion_barrier: Option<Barrier>,
}

pub enum Error {
    MailboxBusy,
}

pub static IPI_RPC_MAILBOXES: Lazy<IpiRpcMailbox> = Lazy::new(IpiRpcMailbox::new);

pub struct IpiRpcMailbox {
    unicast: Box<[AtomicPtr<IpiRpcReq>]>,
    multicast: RwLock<Box<[AtomicPtr<IpiRpcReq>]>>,
    broadcast: AtomicPtr<IpiRpcReq>,
}

impl IpiRpcMailbox {
    pub fn new() -> Self {
        Self {
            unicast: make_boxed_slice(get_lp_count() as usize, || {
                AtomicPtr::<IpiRpcReq>::new(core::ptr::null_mut())
            }),
            multicast: RwLock::new(make_boxed_slice(get_lp_count() as usize, || {
                AtomicPtr::<IpiRpcReq>::new(core::ptr::null_mut())
            })),
            broadcast: AtomicPtr::<IpiRpcReq>::new(core::ptr::null_mut()),
        }
    }

    pub fn try_write_unicast(&self, dest: LpId, req: *mut IpiRpcReq) -> Result<(), Error> {
        let result = self.unicast[dest as usize].compare_exchange(
            core::ptr::null_mut(),
            req,
            AcqRel,
            Acquire,
        );
        if result.is_ok() {
            Ok(())
        } else {
            Err(Error::MailboxBusy)
        }
    }

    pub fn try_write_multicast(&self, dest: Vec<LpId>, req: *mut IpiRpcReq) -> Result<(), Error> {
        todo!()
    }

    pub fn try_write_broadcast(&self, req: *mut IpiRpcReq) -> Result<(), Error> {
        let result = self.broadcast.compare_exchange(core::ptr::null_mut(), req, AcqRel, Acquire);
        if result.is_ok() {
            Ok(())
        } else {
            Err(Error::MailboxBusy)
        }
    }

    pub fn read_own_unicast(&self) -> *mut IpiRpcReq {
        self.unicast[get_lp_id!() as usize].load(Acquire)
    }

    pub fn read_own_multicast(&self) -> *mut IpiRpcReq {
        self.multicast.read()[get_lp_id!() as usize].load(Acquire)
    }

    pub fn read_broadcast(&self) -> *mut IpiRpcReq {
        self.broadcast.load(Acquire)
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
