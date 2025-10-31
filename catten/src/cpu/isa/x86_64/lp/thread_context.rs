use core::mem::offset_of;
use core::ops::Add;

use crate::memory::{AddressSpaceId, VAddr};

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ThreadContext {
    pub cr3: u64,
    pub rsp_cpl0: u64,
    /*The interrupt stack frame contains all the information necessary to resume execution using an `iret`.
      This works somewhat differently depending on whether the interrupt happened in user mode or kernel mode.
      In both cases, the CPU pushes the following values onto the stack (in this order):
        - Stack Segment Selector (padded to 8 bytes)
        - Stack Pointer (RSP)
        - RFLAGS
        - Code Segment Selector (padded to 8 bytes)
        - Instruction Pointer (RIP)
      If the interrupt happened at CPL=3 the LP switches RSP to the value defined for RSP0 in the TSS before pushing these values.
      Thus, when returning from the interrupt, the CPU will restore RSP to the user stack pointer saved in the interrupt stack frame.
      If the interrupt happened at CPL=0, RSP remains unchanged.
      Thus for user threads we need two separate stacks: one for CPL=0 (kernel mode) and one for CPL=3 (user mode) whereas
      for kernel threads we only need one stack (CPL=0).
    */
}

#[repr(C, packed)]
struct IretFrame {
    rip: u64,
    cs_sel: u64,
    rflags: u64,
    rsp: u64,
    ss_sel: u64,
    // General purpose registers excluding RSP
    gprs: [u64; 15],
}

impl ThreadContext {
    pub fn new(asid: AddressSpaceId, stack_top: VAddr, entry_point: VAddr) -> Self {
        todo!(
            "Write an iret frame to the stack at stack_top - size_of::<IretFrame>() and set \
             rsp_cpl0 accordingly. For user threads, rsp_cpl0 should also be written to the TSS \
             when the context is loaded."
        );
        ThreadContext {
            cr3: asid as u64,
            rsp_cpl0: stack_top.into(),
        }
    }

    pub fn get_asid(&self) -> AddressSpaceId {
        self.cr3 as AddressSpaceId
    }
}

#[unsafe(no_mangle)]
pub static TC_CR3_OFFSET: usize = offset_of!(ThreadContext, cr3);
#[unsafe(no_mangle)]
pub static TC_RSP_CPL0_OFFSET: usize = offset_of!(ThreadContext, rsp_cpl0);
