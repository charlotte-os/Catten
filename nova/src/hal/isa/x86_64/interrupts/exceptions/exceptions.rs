//! X86_64 cpu exception handlers, this code is how it is by design,
//! the calling convention for these is special, and needs to be naked in pure asm
//! as a consequence.
//!

use core::arch::global_asm;

#[no_mangle]
static mut BSP_REGS: [u64; 16] = [0; 16];

global_asm! {
    include_str!("exceptions.asm"),
}

// Handlers
extern "C" {
    fn ih_divide_by_zero();
    fn ih_double_fault();
    fn ih_general_protection_fault();
    fn ih_page_fault();
    fn ih_segment_not_present();
    fn ih_debug();
    fn ih_non_maskable_interrupt();
    fn ih_breakpoint();
    fn ih_overflow();
    fn ih_bound_range_exceeded();
    fn ih_invalid_opcode();
    fn ih_device_not_available();
    fn ih_invalid_tss();
    fn ih_stack_segment_fault();
    fn ih_reserved();
    fn ih_x87_floating_point();
    fn ih_alignment_check();
    fn ih_machine_check();
    fn ih_simd_floating_point();
    fn ih_virtualization();
    fn ih_control_protection();
    fn ih_hypervisor_injection();
    fn ih_vmm_communication();
    fn ih_security_exception();
}
