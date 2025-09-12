pub mod lp_local;
pub mod thread;

use core::arch::naked_asm;

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

    #[unsafe(naked)]
    extern "C" fn switch_context() {
        naked_asm!(
            // The invoking context is expected to have pushed RIP to the stack already.
            // This routine should only ever be invoked via an interrupt as it returns via `iretq`.
            /*Save the current thread's context*/
            // Save the flags register
            "pushfq",
            // Save all general-purpose registers except for the stack pointer
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
            // Save the page table base register (CR3)
            "mov rax, cr3",
            "push rax",
            // save the stack pointer to the thread control block
            "call get_current_tid",
            "mov rdi, rax",
            "mov rsi, rsp",
            "call write_thread_stack_ptr",

            /*Load the next thread's context*/
            // load the stack pointer of the next thread to be executed
            "call get_next_tid",
            "mov rdi, rax",
            "call read_thread_stack_ptr",
            "mov rsp, rax",
            // load the page table base register (CR3)
            "sub rsp, 8",
            "mov rax, [rsp]",
            "mov cr3, rax",
            // load all of the other GPRs
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rbp",
            "pop rdi",
            "pop rsi",
            "pop rdx",
            "pop rcx",
            "pop rbx",
            "pop rax",
            // Restore the flags register
            "popfq",
            // Return to the loaded thread's execution path
            "iretq"
        );
    }

    #[unsafe(naked)]
    extern "C" fn load_context() {
        naked_asm!(
            // load the stack pointer of the next thread to be executed
            "call get_next_tid",
            "mov rdi, rax",
            "call read_thread_stack_ptr",
            "mov rsp, rax",
            // load all of the other GPRs
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rbp",
            "pop rdi",
            "pop rsi",
            "pop rdx",
            "pop rcx",
            "pop rbx",
            "pop rax",
            // Restore the flags register
            "popfq",
            // Jump to the loaded thread's entry point at the correct privilege level
            "iretq"
        );
    }

    #[unsafe(naked)]
    extern "C" fn enter_initial_thread_context(
        new_stack: *const [u8],
        entry_point: extern "C" fn() -> !,
    ) -> ! {
        // create a fake interrupt frame on the stack and iretq to the entry point
        naked_asm!{
            "movzx rax, word ptr [rip + KERNEL_DATA_SELECTOR]",
            "push rax",
            "mov rax, rsp",
            "add rax, 8",
            "push rax", // stack pointer above this fake interrupt frame (x86 stack grows down)
            "pushfq",
            "movzx rax, word ptr [rip + KERNEL_CODE_SELECTOR]",
            "push rax",
            "push rdi", // entry_point
            "iretq"
        }
    }
}
