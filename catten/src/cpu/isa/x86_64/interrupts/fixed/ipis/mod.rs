core::arch::global_asm!(include_str!("ipis.asm"));

unsafe extern "C" {
    pub fn isr_interprocessor_interrupt();
}
