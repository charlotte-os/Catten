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

//The actual ISRs
.global isr_divide_by_zero
isr_divide_by_zero:
	// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_divide_by_zero

	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_double_fault
isr_double_fault:
	//Registers are not saved since this exception is an abort
	pop rdi //pop the error code (should always be 0)
	call ih_double_fault
	hlt //halt the core since double faults are an abort

.global isr_general_protection_fault
isr_general_protection_fault:
	pop rdi //pop the error code
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_general_protection_fault
	hlt
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_page_fault
isr_page_fault:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	pop rdi //pop the error code
	call ih_page_fault
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_segment_not_present
isr_segment_not_present:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	pop rdi // Pop the error code into RDI for the handler
	call ih_segment_not_present
	push rdi // Push the error code back onto the stack for restoring context
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	add rsp, 8 // Clean up the error code from the stack
	iretq

.global isr_debug
isr_debug:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_debug
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_non_maskable_interrupt
isr_non_maskable_interrupt:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_non_maskable_interrupt
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_breakpoint
isr_breakpoint:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_breakpoint
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq


.global isr_overflow
isr_overflow:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_overflow
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_bound_range_exceeded
isr_bound_range_exceeded:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_bound_range_exceeded
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_invalid_opcode
isr_invalid_opcode:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_invalid_opcode
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_device_not_available
isr_device_not_available:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_device_not_available
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_invalid_tss
isr_invalid_tss:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	pop rdi
	call ih_invalid_tss
	push rdi
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	add rsp, 8
	iretq

.global isr_stack_segment_fault
isr_stack_segment_fault:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	pop rdi
	call ih_stack_segment_fault
	push rdi
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	add rsp, 8
	iretq

.global isr_reserved
isr_reserved:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	// No error code to pop for this vector, as it's not used
	call ih_reserved
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_x87_floating_point
isr_x87_floating_point:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_x87_floating_point
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_alignment_check
isr_alignment_check:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	pop rdi
	call ih_alignment_check
	push rdi
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

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
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_simd_floating_point
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_virtualization
isr_virtualization:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_virtualization
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_control_protection
isr_control_protection:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	pop rdi
	call ih_control_protection
	push rdi
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	add rsp, 8
	iretq

.global isr_hypervisor_injection
isr_hypervisor_injection:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	call ih_hypervisor_injection
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	iretq

.global isr_vmm_communication
isr_vmm_communication:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	pop rdi // Pop the error code into RDI for the handler
	call ih_vmm_communication
	push rdi // Push the error code back onto the stack for correct stack alignment
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	add rsp, 8 // Clean up the error code from the stack
	iretq

.global isr_security_exception
isr_security_exception:
		// save the caller saved registers
	push rax
	push rdi
	push rsi
	push rdx
	push rcx
	push r8
	push r9
	push r10
	push r11

	pop rdi // Pop the error code into RDI for the handler
	call ih_security_exception
	push rdi // Push the error code back onto the stack for correct stack alignment
	
	// restore the caller saved registers
	pop r11
	pop r10
	pop r9
	pop r8
	pop rcx
	pop rdi
	pop rsi
	pop rax

	add rsp, 8 // Clean up the error code from the stack
	iretq