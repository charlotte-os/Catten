unsafe extern "custom" {
    pub unsafe fn isr_context_switch();
    pub unsafe fn enter_init_thread_ctx();
}
core::arch::global_asm!(include_str!("context_switch.asm"));

pub extern "C" fn set_next_thread() {
    todo!(
        "Get the next thread ID from the local scheduler and load a pointer to the thread's \
         context into FSBASE"
    )
}
