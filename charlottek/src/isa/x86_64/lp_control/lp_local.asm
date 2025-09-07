.macro save_cr3_to_gs
mov rax, cr3
mov gs:[0], rax
.endm

.macro save_rsp_to_gs
mov rax, rsp
mov gs:[8], rax
.endm

.macro load_cr3_from_gs
mov rax, gs:[0]
mov cr3, rax
.endm

.macro load_rsp_from_gs
mov rax, gs:[8]
mov rsp, rax
.endm