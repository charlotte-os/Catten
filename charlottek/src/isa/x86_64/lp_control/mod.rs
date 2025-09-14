pub mod core_state;
pub mod lp_local;

use core::arch::{asm, naked_asm};

pub use crate::isa::interface::lp_control::LpControlIfce;

pub enum Error {}

pub struct LpControl;

impl LpControlIfce for LpControl {
    type Error = Error;
    // The logical processor ID is a 32-bit value on x86_64, representing the xAPIC ID in x2APIC
    // mode.
    type LpId = u32;

    #[unsafe(naked)]
    #[unsafe(no_mangle)]
    extern "C" fn halt() -> ! {
        naked_asm!(
            "hlt",
            "jmp halt"
        );
    }

    #[unsafe(naked)]
    extern "C" fn mask_interrupts() {
        naked_asm!{
            "cli",
            "ret"
        };
    }

    #[unsafe(naked)]
    extern "C" fn unmask_interrupts() {
        naked_asm!{
            "sti",
            "ret"
        };
    }

    #[unsafe(naked)]
    extern "C" fn get_lp_id() -> Self::LpId {
        // Read the LAPIC ID using the x2APIC MSR interface.
        naked_asm!(
            "mov ecx, 0x802",
            "rdmsr",
            "ret"
        );
    }
}

macro_rules! exit_context {
    () => {
        unsafe {
            asm!{
                                "push rax",
                                "push rbx",
                                "push rcx",
                                "push rdx",
                                "push rsi",
                                "push rdi",
                                "push rbp",
                                "push r8",
                                "push r9",
                                "push r10",
                                "push r11",
                                "push r12",
                                "push r13",
                                "push r14",
                                "push r15",
                                "push rsp",
                                "call get_current_tid",
                                "mov rdi, rax",
                                "call save_core_state",
                            };
        }
    };
}
