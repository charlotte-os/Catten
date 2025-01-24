use core::arch::{asm, global_asm};

// Include the interrupt vector table assembly
global_asm!(include_str!("ivt.asm"));

#[inline(always)]
pub fn load_ivt() {
    // Load the interrupt vector table
    unsafe {
        // Load the interrupt vector table
        asm!("ldr x0, =ivt", "msr vbar_el1, x0");
    }
}
#[no_mangle]
pub extern "C" fn sync_dispatcher() {}
#[no_mangle]
pub extern "C" fn irq_dispatcher() {}
#[no_mangle]
pub extern "C" fn fiq_dispatcher() {}
#[no_mangle]
pub extern "C" fn serr_dispatcher() {}
