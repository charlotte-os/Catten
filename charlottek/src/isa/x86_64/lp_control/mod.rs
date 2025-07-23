use core::arch::{asm, naked_asm};

use crate::isa::interface::lp_control::LpControlIfce;
use crate::isa::x86_64::init::gdt;

pub enum Error {}

pub struct LpControl;

impl LpControlIfce for LpControl {
    type Error = Error;
    // The logical processor ID is a 32-bit value on x86_64, representing the xAPIC ID in x2APIC
    // mode.
    type LpId = u32;

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
    extern "C" fn switch_context() {
        #[rustfmt::skip] // keep each instruction on a separate line
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
            "pop rax",
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
        #[rustfmt::skip] // keep each instruction on a separate line
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

    fn init_new_thread_stack(stack: &mut [u8], entry_point: fn() -> !, user_mode: bool) {
        let mut stack_ptr = unsafe { stack.as_mut_ptr().byte_offset(stack.len() as isize) };
        stack_ptr = (stack_ptr as usize & !0xf) as *mut u8; // Align to 16 bytes as an AMD64 processor would when entering an interrupt
        let stack_base = stack_ptr;
        // Push the data segment selector for the stack segment
        stack_ptr = stack_ptr.wrapping_byte_sub(8);
        unsafe {
            *(stack_ptr as *mut u64) = if user_mode {
                gdt::USER_DATA_SELECTOR as u64
            } else {
                gdt::KERNEL_DATA_SELECTOR as u64
            };
        }
        // Push the stack base address
        stack_ptr = stack_ptr.wrapping_byte_sub(8);
        unsafe {
            *(stack_ptr as *mut *mut u8) = stack_base;
        }
        // Push RFLAGS
        stack_ptr = stack_ptr.wrapping_byte_sub(8);
        unsafe {
            asm!(
                "push rax", // Save RAX to avoid clobbering
                "pushfq",
                "pop rax",
                "mov [{stack_ptr}], rax",
                "pop rax", // Restore RAX
                stack_ptr = in(reg) stack_ptr
            );
        }
        // Push the code segment selector
        stack_ptr = stack_ptr.wrapping_byte_sub(8);
        unsafe {
            if user_mode {
                // User mode code segment selector (0x23)
                *(stack_ptr as *mut u64) = gdt::USER_CODE_SELECTOR as u64;
            } else {
                // Kernel mode code segment selector (0x08)
                *(stack_ptr as *mut u64) = gdt::KERNEL_CODE_SELECTOR as u64;
            }
        }
        // Push the instruction pointer (entry point)
        stack_ptr = stack_ptr.wrapping_byte_sub(8);
        unsafe {
            *(stack_ptr as *mut fn() -> !) = entry_point;
        }
    }
}
