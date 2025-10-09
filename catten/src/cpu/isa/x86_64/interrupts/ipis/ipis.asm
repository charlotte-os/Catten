.code64

.section .text
.global isr_interprocessor_interrupt
isr_interprocessor_interrupt:
mov rdi, gs:[rip + GS_OFFSET_IPI_QUEUE]
call ih_interprocessor_interrupt
iretq