.code64

.data
gdtr: 
	.2byte 447 // Place 447 = (64 * 7) - 1 in the gdtr label
	.8byte 0

.text
.global asm_load_gdt
asm_load_gdt:
	mov [rip + gdtr + 2], rdi
	lgdt [rip + gdtr]
	ret

.global asm_reload_segment_regs
asm_reload_segment_regs:
	mov rax, 1 //segment descriptor 1 is the kernel code segment
	shl rax, 3
	push rax
	lea rax, [rip + reload_cs]
	push rax
	retfq

reload_cs:
	mov ax, 2 //segment descriptor 2 is the kernel data segment
	shl ax, 3
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