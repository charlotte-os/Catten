use alloc::boxed::Box;
use core::option::Option;
use core::ptr::NonNull;

use crate::process_mgmt::{PROCESS_TABLE, Process};
use crate::threading::thread::ThreadGuid;

/* This structure contains the global variables that are local to the current logical processor */
struct LpLocal {
    /*The LP local scheduler gives a thread it wants to run to the current logical processor by moving its owning pointer
    into this structure. When it is done with it the pointer is either returned to the execution ring or handed off to the global scheduler to be blocked */
    curr_thread: Option<Box<Thread>>,
}

impl LpLocal {
    pub fn new() -> Self {
        LpLocal {
            curr_thread: None,
        }
    }
}
