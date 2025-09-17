.code 64


.section .text
.global switch_context
switch_context:
// save GPRs except rsp to the gs segment
    mov gs:[0], r15
    mov gs:[8], r14
    mov gs:[16], r13
    mov gs:[24], r12
    mov gs:[32], r11
    mov gs:[40], r10
    mov gs:[48], r9
    mov gs:[56], r8
    mov gs:[64], rdi
    mov gs:[72], rsi
    mov gs:[80], rbp
    mov gs:[88], rdx
    mov gs:[96], rcx
    mov gs:[104], rbx
    mov gs:[112], rax
// save the control registers to the gs segment
    mov gs:[120], cr0
    mov gs:[136], cr3
    mov gs:[144], cr4