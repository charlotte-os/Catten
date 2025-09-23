.code64

.section .text
.global isr_ipi
isr_ipi:
mov rdi, gs:[rip + GS_OFFSET_IPI_MAILBOX]
call ih_ipi
iretq