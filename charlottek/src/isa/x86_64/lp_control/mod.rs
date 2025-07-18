use alloc::boxed::Box;
use alloc::collections::btree_map::BTreeMap;
use alloc::collections::vec_deque::VecDeque;
use core::arch::{asm, naked_asm};
use core::f64::RADIX;

use crate::isa::interface::lp_control::LpControlIfce;
use crate::tsmp::threading::{ThreadId, get_current_tid, get_thread_context};

static mut TEMP_STATE: BTreeMap<<LpControl as LpControlIfce>::LpId, LpState> = BTreeMap::new();

pub enum Error {}

pub struct LpControl;

impl LpControlIfce for LpControl {
    type Error = Error;
    // The logical processor ID is a 32-bit value on x86_64, representing the xAPIC ID in x2APIC
    // mode.
    type LpId = u32;
    type LpState = LpState;

    #[inline(always)]
    fn halt() -> ! {
        unsafe {
            asm!("hlt");
        }
        loop {}
    }

    #[inline(always)]
    fn mask_interrupts() {
        unsafe {
            asm!("cli");
        }
    }

    #[inline(always)]
    fn unmask_interrupts() {
        unsafe {
            asm!("sti");
        }
    }

    #[inline(always)]
    fn get_lp_id() -> Self::LpId {
        let lp_id: Self::LpId;
        // Read the LAPIC ID using the x2APIC MSR interface.
        unsafe {
            asm!(
                "mov ecx, 0x802",
                "rdmsr",
                out("eax") lp_id
            );
        }
        lp_id
    }

    #[unsafe(naked)]
    extern "C" fn switch_context() -> ThreadId {
        naked_asm!(
            "push rax",
            "push rdi",
            "call get_current_tid",
            "mov rdi, rax",
            "call get_thread_context",
            "pop rdi",
            // a pointer to the thread context is now in rax
            // write all GPRs to the thread context
            "mov [rax + 1 * 8], rbx",
            "mov [rax + 2 * 8], rcx",
            "mov [rax + 3 * 8], rdx",
            "mov [rax + 4 * 8], rsi",
            "mov [rax + 5 * 8], rdi",
            // The stack pointer must be restored to the value it had before this routine, saved,
            // and then restored after the context is saved.
            "add rsp, 8", // Adjust stack pointer to remove the pushed registers
            "mov [rax + 6 * 8], rsp",
            "sub rsp, 8", // Restore stack the pointer
            "mov [rax + 7 * 8], rbp",
            "mov [rax + 8 * 8], r8",
            "mov [rax + 9 * 8], r9",
            "mov [rax + 10 * 8], r10",
            "mov [rax + 11 * 8], r11",
            "mov [rax + 12 * 8], r12",
            "mov [rax + 13 * 8], r13",
            "mov [rax + 14 * 8], r14",
            "mov [rax + 15 * 8], r15",
            // save the original rax value from the stack
            "pop rbx",
            "mov [rax + 0 * 8], rbx",
            // Save the instruction pointer and flags
            "pop rbx",                 // get the return address from the stack
            "mov [rax + 16 * 8], rbx", // rip
            "pushfq",                  // push the flags onto the stack
            "pop rbx",                 // pop the flags into rbx
            "mov [rax + 17 * 8], rbx", // rflags
        )
    }

    extern "C" fn load_lp_state(tid: ThreadId);
}

const GPR_COUNT: usize = 16; // Number of general-purpose registers on x86_64.

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct LpState {
    gprs: [u64; GPR_COUNT],
    rip: u64,
    rflags: u64,
}
