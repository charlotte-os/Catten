.section .bss
temp_lp_state:
; General Purpose Registers + RFLAGS + RIP
.space 18 * 8

.section .data
temp_lp_state_me:
; temporary LP state mutex
.byte 0

.global asm_save_lp_state
asm_save_lp_state:
; Save the current LP state to temp_lp_state
    pushfq ; Save RFLAGS before taking the mutex
    push rax ; Save RAX to use it for cmpxchg
    push rcx ; Save RCX to use it for cmpxchg
    xor rax, rax ; set RAX to 0 for cmpxchg
    mov rcx, 1 ; Set RCX to 1 for cmpxchg
wait_me:
    lock cmpxchg temp_lp_state_me, cl ; Try to acquire the mutex
    jnz wait_me ; If mutex is not acquired, spin until it is
    pop rcx ; Restore RCX
    pop rax ; Restore RAX
    ; Save the general-purpose registers
    mov [temp_lp_state + 0 * 8], rax
    mov [temp_lp_state + 1 * 8], rbx
    mov [temp_lp_state + 2 * 8], rcx
    mov [temp_lp_state + 3 * 8], rdx
    mov [temp_lp_state + 4 * 8], rsi
    mov [temp_lp_state + 5 * 8], rdi
    mov [temp_lp_state + 7 * 8], rbp
    mov [temp_lp_state + 8 * 8], r8
    mov [temp_lp_state + 9 * 8], r9
    mov [temp_lp_state + 10 * 8], r10
    mov [temp_lp_state + 11 * 8], r11
    mov [temp_lp_state + 12 * 8], r12
    mov [temp_lp_state + 13 * 8], r13
    mov [temp_lp_state + 14 * 8], r14
    mov [temp_lp_state + 15 * 8], r15
    ; save RFLAGS
    pop rax
    mov [temp_lp_state + 16 * 8], rax
    ; save the stack pointer after matching pushes and pops
    mov [temp_lp_state + 6 * 8], rsp
    ; save RIP
    mov rax, [rsp] ; Get the return address (RIP)
    mov [temp_lp_state + 17 * 8], rax
    ; return, the mutex is released in the caller
    ret