use core::ops::DerefMut;
use core::ptr::NonNull;

use crate::process_mgmt::{PROCESS_TABLE, Process};
use crate::threading::thread::ThreadGuid;

/* This structure contains the global data for the current logical processor */
struct LpGlobal {
    usr_thread: Option<ThreadGuid>,
}

impl LpGlobal {
    pub fn new() -> Self {
        LpGlobal {
            usr_thread: None,
        }
    }
}
