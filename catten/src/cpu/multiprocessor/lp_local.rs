//! # Logical Processor Local Data
//!
//! The LP local structure contains data that is specific to each logical processor (LP) in a
//! multiprocessor system. ISA specific data is stored in the `isa_data` field, while platform
//! independent fields are stored directly in the `LpLocal` struct.

use alloc::collections::vec_deque::VecDeque;
use alloc::sync::Arc;
use core::ffi::c_int;

use spin::Mutex;

use crate::cpu::isa::interrupts::ipis::IpiRpcReq;
use crate::cpu::isa::lp::IsaData;

pub struct LpLocal {
    /// ISA specific data for the logical processor.
    pub isa_data: IsaData,
    pub c_errno: c_int,
    pub ipi_req_queue: Arc<Mutex<VecDeque<IpiRpcReq>>>,
}
