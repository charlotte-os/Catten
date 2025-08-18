pub mod scheduler;
pub mod sync;

use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet};

use spin::Mutex;

use crate::isa::current_isa::lp_control::LpControl;
use crate::isa::interface::lp_control::LpControlIfce;
use crate::isa::interface::memory::address::{Address, VirtualAddress};
use crate::memory::vmem::{AddressSpaceId, VAddr};
use crate::multiprocessing::{LP_TABLE, LpId, LpState};

pub type ThreadId = u64;

static mut THREAD_TABLE: BTreeMap<ThreadId, Mutex<ThreadControlBlock>> = BTreeMap::new();

pub enum ThreadingError {}

pub fn get_current_thread_id() -> Option<ThreadId> {
    let lp_id = LpControl::get_lp_id();
    if let Some(tcb) = LP_TABLE.get(&lp_id) {
        if let LpState::Running(tid) = *tcb.lock() {
            return Some(tid);
        }
    }
    None
}

#[unsafe(no_mangle)]
pub extern "C" fn cabi_get_current_thread_id(dest: &mut ThreadId) -> i64 {
    if let Some(tid) = get_current_thread_id() {
        *dest = tid;
        0i64
    } else {
        -1i64
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn get_next_thread_id() -> ThreadId {
    todo!("Thread scheduling not implemented yet");
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
    address_space: AddressSpaceId,
    #[allow(unused)]
    user_mode: bool,
    #[allow(unused)]
    stack: Box<[u8]>,
    pub stack_pointer: VAddr,
    pub lp_affinity_set: BTreeSet<LpId>,
}

impl ThreadControlBlock {
    pub fn new(
        address_space: AddressSpaceId,
        entry_point: fn() -> !,
        stack_size: usize,
        user_mode: bool,
    ) -> Self {
        let mut stack = alloc::vec![0u8; stack_size].into_boxed_slice();
        let stack_ptr = <VAddr as VirtualAddress>::from_ptr(unsafe {
            stack.as_ptr().byte_offset(stack_size as isize)
        });

        LpControl::init_new_thread_stack(stack.as_mut(), entry_point, user_mode);

        ThreadControlBlock {
            address_space,
            user_mode,
            stack,
            stack_pointer: stack_ptr,
            lp_affinity_set: BTreeSet::new(),
        }
    }
}
