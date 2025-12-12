/* Add the following line in any x86-64 assembly source to use the macros defined here.
   
   .include catten/src/cpu/isa/x86_64/asm_macros/context_switch.asm
 */

.macro ctx_save_m
    // Save the general purpose registers to the interrupt stack
    push r15
    push r14
    push r13
    push r12
    push r11
    push r10
    push r9
    push r8
    push rsi
    push rdi
    push rbp
    push rdx
    push rcx
    push rbx
    push rax
    // compute the address of the thread context in the logical processor's local data segment
    mov rax, LP_LOCAL_THREAD_OFFSET
    add rax, THREAD_TC_OFFSET
    // save the stack pointer for CPL0 into the thread context for later restoration
    mov gs:[rip + rax + TC_RSP_CPL0_OFFSET], rsp
    // Save the current page table hierarchy base into the thread context
    mov rdx, cr3
    mov gs:[rip + rax + TC_CR3_OFFSET], rdx
.endm

.macro ctx_load_m
    mov rax, LP_LOCAL_THREAD_OFFSET
    add rax, THREAD_TC_OFFSET
    // Load the correct page table hierarchy into the page table base control register, CR3
    mov rdx, gs:[rip + rax + TC_CR3_OFFSET]
    mov cr3, rdx
    // Load the stack pointer for CPL0 from the thread context to restore kernel stack 
    // with the register state and interrupt return frame
    mov rsp, gs:[rip + rax + TC_RSP_CPL0_OFFSET]
    // write rsp for CPL=0 as it will be at the time of iretq to the TSS
    mov rdi, [rsp + 19 * 8] // 15 general purpose registers + 4 quadwords of iretq frame
    call write_rsp0
    // Restore the general purpose registers
    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rbp
    pop rdi
    pop rsi
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15
.endm

.macro m_save_gprs
    push r15
    push r14
    push r13
    push r12
    push r11
    push r10
    push r9
    push r8
    push rsi
    push rdi
    push rbp
    push rdx
    push rcx
    push rbx
    push rax
.endm

.macro m_restore_gprs
    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rbp
    pop rdi
    pop rsi
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15
.endm