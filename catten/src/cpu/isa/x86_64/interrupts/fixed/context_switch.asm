.bits 64

.section .text
.global switch_context
isr_switch_context:
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15
    mov fs:[0], rsp  # Save current stack pointer to the current thread's context
    call set_next_thread  # Call scheduler to get the next thread and set fs:[0] to its context base
    mov rsp, fs:[0]  # Load the next thread's stack pointer from its context
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    iretq

.global enter_init_thread_ctx
enter_init_thread_ctx:
    call set_next_thread  # Call scheduler to get the next thread and set fs:[0] to its context base
    mov rsp, fs:[0]  # Load the next thread's stack pointer from its context
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    ret