.code 64

.section .text
.global isr_ipi
mov rdi, gs:[GS_IPI_MAILBOX_OFFSET]
call ih_ipi
iretq