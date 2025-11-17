#[unsafe(naked)]
pub extern "C" fn isr_spurious() {
    // Spurious interrupt handler does nothing
    core::arch::naked_asm! {
        "iretq"
    };
}
