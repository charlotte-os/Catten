.code64

.text
//Handlers
.extern ih_divide_by_zero
.extern ih_double_fault
.extern ih_general_protection_fault
.extern ih_page_fault
.extern ih_segment_not_present
.extern ih_debug
.extern ih_non_maskable_interrupt
.extern ih_breakpoint
.extern ih_overflow
.extern ih_bound_range_exceeded
.extern ih_invalid_opcode
.extern ih_device_not_available
.extern ih_invalid_tss
.extern ih_stack_segment_fault
.extern ih_reserved
.extern ih_x87_floating_point
.extern ih_alignment_check
.extern ih_machine_check
.extern ih_simd_floating_point
.extern ih_virtualization
.extern ih_control_protection
.extern ih_hypervisor_injection
.extern ih_vmm_communication
.extern ih_security_exception

save_regs:
	push rax
	push rbx
	push rcx
	push rdx
	push rsi
	push rdi
	push rsp
	push rbp
	push r8
	push r9
	push r10
	push r11
	push r12
	push r13
	push r14
	push r15
	ret
restore_regs:
	pop r15
	pop r14
	pop r13
	pop r12
	pop r11
	pop r10
	pop r9
	pop r8
	pop rbp
	pop rsp
	pop rdi
	pop rsi
	pop rdx
	pop rcx
	pop rbx
	pop rax
	ret

//The actual ISRs
.global isr_divide_by_zero
isr_divide_by_zero:
	call save_regs
	call ih_divide_by_zero
	call restore_regs
	iretq

.global isr_double_fault
isr_double_fault:
	//Registers are not saved since this exception is an abort
	pop rdi //pop the error code (should always be 0)
	call ih_double_fault
	hlt //halt the core since double faults are an abort

.global isr_general_protection_fault
isr_general_protection_fault:
	call save_regs
	pop rdi //pop the error code
	mov rsi, [rip] // Save the RIP to RSI for the handler
	call ih_general_protection_fault
	hlt
	call restore_regs
	iretq

.global isr_page_fault
isr_page_fault:
	call save_regs
	pop rdi //pop the error code
	call ih_page_fault
	call restore_regs
	iretq

.global isr_segment_not_present
isr_segment_not_present:
	call save_regs
	pop rdi // Pop the error code into RDI for the handler
	call ih_segment_not_present
	push rdi // Push the error code back onto the stack for restoring context
	call restore_regs
	add rsp, 8 // Clean up the error code from the stack
	iretq

.global isr_debug
isr_debug:
	call save_regs
	call ih_debug
	call restore_regs
	iretq

.global isr_non_maskable_interrupt
isr_non_maskable_interrupt:
	call save_regs
	call ih_non_maskable_interrupt
	call restore_regs
	iretq

.global isr_breakpoint
isr_breakpoint:
	call save_regs
	call ih_breakpoint
	call restore_regs
	iretq


.global isr_overflow
isr_overflow:
	call save_regs
	call ih_overflow
	call restore_regs
	iretq

.global isr_bound_range_exceeded
isr_bound_range_exceeded:
	call save_regs
	call ih_bound_range_exceeded
	call restore_regs
	iretq

.global isr_invalid_opcode
isr_invalid_opcode:
	call save_regs
	call ih_invalid_opcode
	call restore_regs
	iretq

.global isr_device_not_available
isr_device_not_available:
	call save_regs
	call ih_device_not_available
	call restore_regs
	iretq

.global isr_invalid_tss
isr_invalid_tss:
	call save_regs
	pop rdi
	call ih_invalid_tss
	push rdi
	call restore_regs
	add rsp, 8
	iretq

.global isr_stack_segment_fault
isr_stack_segment_fault:
	call save_regs
	pop rdi
	call ih_stack_segment_fault
	push rdi
	call restore_regs
	add rsp, 8
	iretq

.global isr_reserved
isr_reserved:
	call save_regs
	// No error code to pop for this vector, as it's not used
	call ih_reserved
	call restore_regs
	iretq

.global isr_x87_floating_point
isr_x87_floating_point:
	call save_regs
	call ih_x87_floating_point
	call restore_regs
	iretq

.global isr_alignment_check
isr_alignment_check:
	call save_regs
	pop rdi
	call ih_alignment_check
	push rdi
	call restore_regs
	add rsp, 8
	iretq

.global isr_machine_check
isr_machine_check:
	// Registers are not saved since this exception is an abort
	// Unlike Double Fault, Machine Check does not push an error code
	call ih_machine_check
	hlt // Halt the core since machine checks indicate severe hardware issues

.global isr_simd_floating_point
isr_simd_floating_point:
	call save_regs
	call ih_simd_floating_point
	call restore_regs
	iretq

.global isr_virtualization
isr_virtualization:
	call save_regs
	call ih_virtualization
	call restore_regs
	iretq

.global isr_control_protection
isr_control_protection:
	call save_regs
	pop rdi
	call ih_control_protection
	push rdi
	call restore_regs
	add rsp, 8
	iretq

.global isr_hypervisor_injection
isr_hypervisor_injection:
	call save_regs
	call ih_hypervisor_injection
	call restore_regs
	iretq

.global isr_vmm_communication
isr_vmm_communication:
	call save_regs
	pop rdi // Pop the error code into RDI for the handler
	call ih_vmm_communication
	push rdi // Push the error code back onto the stack for correct stack alignment
	call restore_regs
	add rsp, 8 // Clean up the error code from the stack
	iretq

.global isr_security_exception
isr_security_exception:
	call save_regs
	pop rdi // Pop the error code into RDI for the handler
	call ih_security_exception
	push rdi // Push the error code back onto the stack for correct stack alignment
	call restore_regs
	add rsp, 8 // Clean up the error code from the stack
	iretq