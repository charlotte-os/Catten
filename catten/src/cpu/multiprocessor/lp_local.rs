//! # Logical Processor Local Data
//!
//! The LP local structure contains data that is specific to each logical processor (LP) in a
//! multiprocessor system. ISA specific data is stored in the `isa_data` field, while platform
//! independent fields are stored directly in the `LpLocal` struct.

use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use alloc::sync::Arc;
use core::ffi::c_int;

use spin::Mutex;

use crate::cpu::isa::interface::lp::LpIsaDataIfce;
use crate::cpu::isa::interface::memory::address::VirtualAddress;
use crate::cpu::isa::lp::lp_isa_data::LpIsaData;
use crate::cpu::isa::lp::ops::{get_lp_local_base, set_lp_local_base};
use crate::cpu::multiprocessor::ipi::IpiRpcReq;
use crate::memory::VAddr;

pub enum Error {
    AlreadyInitialized,
}

pub struct LpLocal {
    /// ISA specific data for each logical processor.
    pub isa_data: LpIsaData,
    pub c_errno: c_int,
    pub ipi_req_queue: Arc<Mutex<VecDeque<IpiRpcReq>>>,
}

impl LpLocal {
    fn new() -> Self {
        LpLocal {
            isa_data: <LpIsaData as LpIsaDataIfce>::new(),
            c_errno: 0,
            ipi_req_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn init() -> Result<(), Error> {
        let local_seg_base = get_lp_local_base();
        if local_seg_base != VAddr::from(0u64) {
            Err(Error::AlreadyInitialized)
        } else {
            let lp_local = Box::new(LpLocal::new());
            set_lp_local_base(VAddr::from_mut(Box::into_raw(lp_local)));
            Ok(())
        }
    }

    pub extern "C" fn get() -> &'static LpLocal {
        let ptr: *const LpLocal;
        unsafe {
            core::arch::asm! {
                "lea {}, gs:[0]",
                out(reg) ptr
            }
            core::mem::transmute::<*const LpLocal, &'static LpLocal>(ptr)
        }
    }
}
