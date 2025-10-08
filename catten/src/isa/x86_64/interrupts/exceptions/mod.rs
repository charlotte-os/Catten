use crate::isa::init::gdt;
use crate::isa::interrupts::idt::Idt;
use crate::logln;

pub fn load_exceptions(idt: &mut Idt) {
    idt.set_gate(0, isr_divide_by_zero, gdt::KERNEL_CODE_SELECTOR, true, true);
    idt.set_gate(1, isr_debug, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(2, isr_non_maskable_interrupt, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(3, isr_breakpoint, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(4, isr_overflow, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(5, isr_bound_range_exceeded, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(6, isr_invalid_opcode, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(7, isr_device_not_available, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(8, isr_double_fault, gdt::KERNEL_CODE_SELECTOR, true, true);
    idt.set_gate(10, isr_invalid_tss, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(11, isr_segment_not_present, gdt::KERNEL_CODE_SELECTOR, true, true);
    idt.set_gate(12, isr_stack_segment_fault, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(13, isr_general_protection_fault, gdt::KERNEL_CODE_SELECTOR, true, true);
    idt.set_gate(14, isr_page_fault, gdt::KERNEL_CODE_SELECTOR, true, true);
    idt.set_gate(16, isr_x87_floating_point, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(17, isr_alignment_check, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(18, isr_machine_check, gdt::KERNEL_CODE_SELECTOR, true, true);
    idt.set_gate(19, isr_simd_floating_point, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(20, isr_virtualization, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(21, isr_control_protection, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(28, isr_hypervisor_injection, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(29, isr_vmm_communication, gdt::KERNEL_CODE_SELECTOR, true, false);
    idt.set_gate(30, isr_security_exception, gdt::KERNEL_CODE_SELECTOR, true, false);
}

core::arch::global_asm! {
    include_str!("exceptions.asm"),
}
unsafe extern "C" {
    fn isr_divide_by_zero();
    fn isr_debug();
    fn isr_non_maskable_interrupt();
    fn isr_breakpoint();
    fn isr_overflow();
    fn isr_bound_range_exceeded();
    fn isr_invalid_opcode();
    fn isr_device_not_available();
    fn isr_double_fault();
    fn isr_invalid_tss();
    fn isr_stack_segment_fault();
    fn isr_general_protection_fault();
    fn isr_segment_not_present();
    fn isr_page_fault();
    fn isr_x87_floating_point();
    fn isr_alignment_check();
    fn isr_machine_check();
    fn isr_simd_floating_point();
    fn isr_virtualization();
    fn isr_control_protection();
    fn isr_hypervisor_injection();
    fn isr_vmm_communication();
    fn isr_security_exception();
}

#[unsafe(no_mangle)]
extern "C" fn ih_double_fault(_error_code: u64) {
    logln!("A double fault has occurred in kernelspace! Panicking!");
    panic!("Double fault");
}

#[unsafe(no_mangle)]
extern "C" fn ih_divide_by_zero() {
    logln!("Divide by zero exception occurred!");
    panic!("Divide by zero");
}

#[unsafe(no_mangle)]
extern "C" fn ih_debug() {
    logln!("Debug exception occurred!");
    panic!("Debug exception");
}

#[unsafe(no_mangle)]
extern "C" fn ih_non_maskable_interrupt() {
    logln!("Non-maskable interrupt occurred!");
    panic!("Non-maskable interrupt");
}

#[unsafe(no_mangle)]
extern "C" fn ih_breakpoint() {
    logln!("Breakpoint exception occurred!");
    panic!("Breakpoint exception");
}

#[unsafe(no_mangle)]
extern "C" fn ih_overflow() {
    logln!("Overflow exception occurred!");
    panic!("Overflow exception");
}

#[unsafe(no_mangle)]
extern "C" fn ih_bound_range_exceeded() {
    logln!("Bound range exceeded exception occurred!");
    panic!("Bound range exceeded");
}

#[unsafe(no_mangle)]
extern "C" fn ih_invalid_opcode() {
    logln!("Invalid opcode exception occurred!");
    panic!("Invalid opcode");
}

#[unsafe(no_mangle)]
extern "C" fn ih_device_not_available() {
    logln!("Device not available exception occurred!");
    panic!("Device not available");
}

#[unsafe(no_mangle)]
extern "C" fn ih_invalid_tss() {
    logln!("Invalid TSS exception occurred!");
    panic!("Invalid TSS");
}

#[unsafe(no_mangle)]
extern "C" fn ih_segment_not_present() {
    logln!("Segment not present exception occurred!");
    panic!("Segment not present");
}

#[unsafe(no_mangle)]
extern "C" fn ih_stack_segment_fault() {
    logln!("Stack segment fault occurred!");
    panic!("Stack segment fault");
}

#[unsafe(no_mangle)]
extern "C" fn ih_general_protection_fault(_error_code: u64) {
    logln!("General protection fault occurred!");
    panic!("General protection fault");
}

#[unsafe(no_mangle)]
extern "C" fn ih_page_fault(error_code: u64) {
    logln!("Page fault occurred with error code {:X}!", error_code);
    let pf_addr: u64;
    unsafe {
        core::arch::asm!("mov {0}, cr2", out(reg) pf_addr);
    }
    logln!("Page fault address: {:x}", pf_addr);
    panic!("Page fault");
}

#[unsafe(no_mangle)]
extern "C" fn ih_x87_floating_point() {
    logln!("x87 floating point exception occurred!");
    panic!("x87 floating point exception");
}

#[unsafe(no_mangle)]
extern "C" fn ih_alignment_check() {
    logln!("Alignment check exception occurred!");
    panic!("Alignment check");
}

#[unsafe(no_mangle)]
extern "C" fn ih_machine_check() {
    logln!("Machine check exception occurred!");
    panic!("Machine check");
}

#[unsafe(no_mangle)]
extern "C" fn ih_simd_floating_point() {
    logln!("SIMD floating point exception occurred!");
    panic!("SIMD floating point exception");
}

#[unsafe(no_mangle)]
extern "C" fn ih_virtualization() {
    logln!("Virtualization exception occurred!");
    panic!("Virtualization exception");
}

#[unsafe(no_mangle)]
extern "C" fn ih_control_protection() {
    logln!("Control protection exception occurred!");
    panic!("Control protection exception");
}

#[unsafe(no_mangle)]
extern "C" fn ih_hypervisor_injection() {
    logln!("Hypervisor injection exception occurred!");
    panic!("Hypervisor injection");
}

#[unsafe(no_mangle)]
extern "C" fn ih_vmm_communication() {
    logln!("VMM communication exception occurred!");
    panic!("VMM communication");
}

#[unsafe(no_mangle)]
extern "C" fn ih_security_exception() {
    logln!("Security exception occurred!");
    panic!("Security exception");
}
