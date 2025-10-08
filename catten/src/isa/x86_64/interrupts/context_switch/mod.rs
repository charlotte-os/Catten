core::arch::global_asm!(include_str!("context_switch.asm"));

unsafe extern "C" {
    pub fn isr_switch_thread_context();
}
