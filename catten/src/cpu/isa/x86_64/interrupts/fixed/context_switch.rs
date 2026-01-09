unsafe extern "custom" {
    pub unsafe fn isr_context_switch();
    pub unsafe fn isr_wake_lp();
}
core::arch::global_asm!(include_str!("context_switch.asm"));

use crate::cpu::isa::lp::ops::set_thread_context_ptr;
use crate::cpu::scheduler::system_scheduler::SYSTEM_SCHEDULER;

#[unsafe(no_mangle)]
pub extern "C" fn check_idle_lp() -> bool {
    SYSTEM_SCHEDULER.get_local_scheduler().lock().is_idle()
}

#[unsafe(no_mangle)]
pub extern "C" fn set_next_thread() {
    let next_tid = SYSTEM_SCHEDULER.get_local_scheduler().lock().next();
    unsafe {
        let context_addr = crate::cpu::scheduler::threads::MASTER_THREAD_TABLE
            .try_get_element_arc(next_tid)
            .unwrap()
            .read()
            .get_context_vaddr();
        set_thread_context_ptr(context_addr);
    }
}
