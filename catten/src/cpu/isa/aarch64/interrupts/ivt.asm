// Define the optimized macros
.macro push_volatile_regs
    stp x0, x1, [sp, #-16]!
    stp x2, x3, [sp, #-16]!
    stp x4, x5, [sp, #-16]!
    stp x6, x7, [sp, #-16]!
    stp x8, x9, [sp, #-16]!
    stp x10, x11, [sp, #-16]!
    stp x12, x13, [sp, #-16]!
    stp x14, x15, [sp, #-16]!
    stp x16, x17, [sp, #-16]!
    str x18, [sp, #-8]!
.endm

.macro pop_volatile_regs
    ldr x18, [sp], #8
    ldp x16, x17, [sp], #16
    ldp x14, x15, [sp], #16
    ldp x12, x13, [sp], #16
    ldp x10, x11, [sp], #16
    ldp x8, x9, [sp], #16
    ldp x6, x7, [sp], #16
    ldp x4, x5, [sp], #16
    ldp x2, x3, [sp], #16
    ldp x0, x1, [sp], #16
.endm

.text
.extern sync_dispatcher
.extern irq_dispatcher
.extern fiq_dispatcher
.extern serr_dispatcher
// Interrupt Vector Table
// Given the scheme we empoloy each used IVT entry is 22 instructions exactly while the ISA requires 32 instructions
// This means that we have 10 instructions of padding for each IVT entry
.balign 128
.global ivt
ivt:
// Exception from current EL while using SP_EL0
// Unused because we don't use SP_EL0 as the stack pointer in kernelspace
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
// Exception from current EL using SP_ELx
.balign 128
push_volatile_regs
bl sync_dispatcher
pop_volatile_regs
eret

.balign 128
push_volatile_regs
bl irq_dispatcher
pop_volatile_regs
eret
.balign 128
push_volatile_regs
bl fiq_dispatcher
pop_volatile_regs
eret
.balign 128
push_volatile_regs
bl serr_dispatcher
pop_volatile_regs
eret
// Exception from a lower EL and at least one lower EL is AArch64
.balign 128
push_volatile_regs
bl sync_dispatcher
pop_volatile_regs
eret
.balign 128
push_volatile_regs
bl irq_dispatcher
pop_volatile_regs
eret
.balign 128
push_volatile_regs
bl fiq_dispatcher
pop_volatile_regs
eret
.balign 128
push_volatile_regs
bl serr_dispatcher
pop_volatile_regs
eret
// Exception from a lower EL and all lower ELs are AArch32
// Unused because we don't support AArch32
.balign 128
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
nop
// End of IVT