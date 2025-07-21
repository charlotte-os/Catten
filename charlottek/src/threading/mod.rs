pub mod scheduler;
pub mod sync;

use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet};
use core::mem::MaybeUninit;

use spin::Mutex;

use crate::isa::current_isa::lp_control::LpControl;
use crate::isa::interface::lp_control::LpControlIfce;
use crate::isa::interface::memory::address::{Address, VirtualAddress};
use crate::memory::vmem::VAddr;

pub type ThreadId = u64;
pub type LpId = <LpControl as LpControlIfce>::LpId;

static THREADS_IN_FLIGHT: BTreeMap<LpId, Mutex<MaybeUninit<ThreadId>>> = BTreeMap::new();
static mut THREAD_TABLE: BTreeMap<ThreadId, Mutex<ThreadControlBlock>> = BTreeMap::new();

pub enum ThreadingError {}

#[unsafe(no_mangle)]
pub extern "C" fn get_current_tid() -> ThreadId {
    unsafe {
        THREADS_IN_FLIGHT
            .get(&LpControl::get_lp_id())
            .unwrap()
            .lock()
            .assume_init()
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn get_next_tid() -> ThreadId {
    get_current_tid()
}
#[unsafe(no_mangle)]
pub extern "C" fn write_thread_stack_ptr(tid: ThreadId, sp: VAddr) -> i32 {
    unsafe {
        if let Some(tcb) = THREAD_TABLE.get_mut(&tid) {
            tcb.lock().stack_pointer = sp;
            0i32
        } else {
            -1i32
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn read_thread_stack_ptr(tid: ThreadId) -> VAddr {
    unsafe {
        if let Some(tcb) = THREAD_TABLE.get_mut(&tid) {
            tcb.lock().stack_pointer
        } else {
            VAddr::NULL
        }
    }
}

pub struct ThreadControlBlock {
    user_mode: bool,
    #[allow(unused)]
    stack: Box<[u8]>,
    pub stack_pointer: VAddr,
    pub lp_affinity_set: BTreeSet<LpId>,
}

impl ThreadControlBlock {
    pub fn new(entry_point: fn() -> !, stack_size: usize, user_mode: bool) -> Self {
        let stack = alloc::vec![0u8; stack_size].into_boxed_slice();
        let stack_ptr = <VAddr as VirtualAddress>::from_ptr(unsafe {
            &*stack.as_ptr().byte_offset(stack_size as isize - 1)
        });

        //TODO: Initialize the stack with the entry point function pointer in an ISA-specific
        // manner.

        ThreadControlBlock {
            user_mode,
            stack,
            stack_pointer: stack_ptr,
            lp_affinity_set: BTreeSet::new(),
        }
    }
}
