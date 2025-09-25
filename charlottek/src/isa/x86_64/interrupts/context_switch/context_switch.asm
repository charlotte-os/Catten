.code64

.section .text
.global isr_switch_thread_context
isr_switch_thread_context:
cli
/* Save the current context if needed */
push r15
// Load the pointer to the ThreadContext to be saved from TLS which located at gs:[0].
mov r15, gs:[0]
// If no context to save, skip saving and just load the new context.
cmp r15, 0
je load_context
// Point r15 to the start of the gprs array in the ThreadContext.
add r15, [rip + TC_GPRS_OFFSET]
// Save all general-purpose registers into the ThreadContext.
mov [rip + r15 + 8 * 0], rax
mov [rip + r15 + 8 * 1], rbx
mov [rip + r15 + 8 * 2], rcx
mov [rip + r15 + 8 * 3], rdx
mov [rip + r15 + 8 * 4], rsi
mov [rip + r15 + 8 * 5], rdi
mov [rip + r15 + 8 * 6], rbp
mov [rip + r15 + 8 * 7], rsp
mov [rip + r15 + 8 * 8], r8
mov [rip + r15 + 8 * 9], r9
mov [rip + r15 + 8 * 10], r10
mov [rip + r15 + 8 * 11], r11
mov [rip + r15 + 8 * 12], r12
mov [rip + r15 + 8 * 13], r13
mov [rip + r15 + 8 * 14], r14
pop rax                       // Restore r15 from the stack.
mov [rip + r15 + 8 * 15], rax // Save the original r15 value.
mov r15, gs:[0]
add r15, [rip + TC_CR3_OFFSET] // Point r15 to the cr3 field.
mov rax, cr3           // Read the current CR3 value.
mov [rip + r15], rax   // Save CR3 into the ThreadContext.
/* Load the new context */
load_context:
// Load the pointer to the ThreadContext to be loaded from TLS located at gs:[8].
mov r15, gs:[8]
add r15, [rip + TC_CR3_OFFSET] // Point r15 to the cr3 field.
mov rax, [rip + r15]   // Load CR3 from the ThreadContext.
mov cr3, rax           // Update CR3 to switch address spaces.
mov r15, gs:[8]
add r15, [rip + TC_GPRS_OFFSET] // Point r15 to the start of
// Load all general-purpose registers from the ThreadContext.
mov rax, [rip + r15 + 8 * 0]
mov rbx, [rip + r15 + 8 * 1]
mov rcx, [rip + r15 + 8 * 2]
mov rdx, [rip + r15 + 8 * 3]
mov rsi, [rip + r15 + 8 * 4]
mov rdi, [rip + r15 + 8 * 5]
mov rbp, [rip + r15 + 8 * 6]
mov rsp, [rip + r15 + 8 * 7]
mov r8,  [rip + r15 + 8 * 8]
mov r9,  [rip + r15 + 8 * 9]
mov r10, [rip + r15 + 8 * 10]
mov r11, [rip + r15 + 8 * 11]
mov r12, [rip + r15 + 8 * 12]
mov r13, [rip + r15 + 8 * 13]
mov r14, [rip + r15 + 8 * 14]
mov r15, [rip + r15 + 8 * 15] // Restore the original r15 value.
sti
iret