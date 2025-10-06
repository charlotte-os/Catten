use crate::logln;
use crate::memory::VAddr;
use crate::memory::stack::KERNEL_GUARD_PAGE_BASES;

core::arch::global_asm! {
    include_str!("exceptions.asm"),
}
unsafe extern "C" {
    pub fn isr_divide_by_zero();
    pub fn isr_debug();
    pub fn isr_non_maskable_interrupt();
    pub fn isr_breakpoint();
    pub fn isr_overflow();
    pub fn isr_bound_range_exceeded();
    pub fn isr_invalid_opcode();
    pub fn isr_device_not_available();
    pub fn isr_double_fault();
    pub fn isr_invalid_tss();
    pub fn isr_stack_segment_fault();
    pub fn isr_general_protection_fault();
    pub fn isr_segment_not_present();
    pub fn isr_page_fault();
    pub fn isr_x87_floating_point();
    pub fn isr_alignment_check();
    pub fn isr_machine_check();
    pub fn isr_simd_floating_point();
    pub fn isr_virtualization();
    pub fn isr_control_protection();
    pub fn isr_hypervisor_injection();
    pub fn isr_vmm_communication();
    pub fn isr_security_exception();
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
    let pf_addr_base = VAddr::from(pf_addr as usize).page_base();
    if KERNEL_GUARD_PAGE_BASES.read().contains(&pf_addr_base) {
        logln!("Resizing kernel thread stack with guard page base address {:?}.", pf_addr_base);
        todo!("Actually reallocate the kernel stack that triggered the #PF")
    } else {
        panic!("Page fault");
    }
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
