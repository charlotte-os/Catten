.code64

.extern NULL_SELECTOR
.extern KERNEL_CODE_SELECTOR
.extern KERNEL_DATA_SELECTOR
.extern USER_CODE_SELECTOR
.extern USER_DATA_SELECTOR

.data
gdtr: 
	.2byte 55 // The size of the GDT in bytes (55 bytes for 7 entries) less one byte
	.8byte 0 // The address of the GDT is filled in by the asm_load_gdt function

.text
.global asm_load_gdt
asm_load_gdt:
	mov [rip + gdtr + 2], rdi
	lgdt [rip + gdtr]
	ret

.global asm_reload_segment_regs
asm_reload_segment_regs:
	movzx rax, word ptr [rip + KERNEL_CODE_SELECTOR]
	push rax
	lea rax, [rip + reload_cs]
	push rax
	retfq

reload_cs:
	mov ax, [rip + KERNEL_DATA_SELECTOR]
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax
	ret

.global asm_load_tss
asm_load_tss:
	mov ax, 5
	shl ax, 3
	ltr ax
	ret