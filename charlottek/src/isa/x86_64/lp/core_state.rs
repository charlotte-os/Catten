use core::arch::{asm, naked_asm};
use core::mem::offset_of;

use spin::Lazy;

use crate::isa::x86_64::memory::paging::ADDRESS_SPACE_TABLE;

pub type LpId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct CoreState {
    pub gprs: [u64; 16],
    pub cr3:  u64,
}

const CS_CR3_OFFSET: usize = offset_of!(CoreState, cr3);

extern "C" fn get_kernel_only_cr3() -> u64 {
    ADDRESS_SPACE_TABLE.lock()[&0usize].get_cr3()
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "custom" fn save_core_state_to_fs() {
    // The ABI for this function requires the caller to keep all registers intact from the call
    // site. Upon return the address space is changed to the kernel-only one and all registers are
    // to be treated as clobbered.
    naked_asm!(
        // Save GPRs to FS segment
        "mov fs:[8*0], rax",
        "mov fs:[8*1], rbx", 
        "mov fs:[8*2], rcx",
        "mov fs:[8*3], rdx",
        "mov fs:[8*4], rsi",
        "mov fs:[8*5], rdi",
        "mov fs:[8*6], rbp",
        "mov fs:[8*7], rsp",
        "mov fs:[8*8], r8",
        "mov fs:[8*9], r9",
        "mov fs:[8*10], r10",
        "mov fs:[8*11], r11",
        "mov fs:[8*12], r12",
        "mov fs:[8*13], r13",
        "mov fs:[8*14], r14",
        "mov fs:[8*15], r15",
        // Save current CR3
        "mov rax, cr3",
        "mov fs:[{CS_CR3_OFFSET}], rax",
        // Switch to kernel-only address space
        "call get_kernel_only_cr3",
        "mov cr3, rax",
        CS_CR3_OFFSET = const CS_CR3_OFFSET
    )
}
