#[unsafe(naked)]
pub unsafe extern "custom" fn isr_spurious() {
    // Spurious interrupt handler does nothing
    core::arch::naked_asm! {
        "iretq"
    };
}
