use core::arch::naked_asm;
use core::mem::offset_of;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ThreadContext {
    pub rsp_cpl0: u64,
    pub cr3: u64,
}

#[unsafe(no_mangle)]
pub static TC_RSP_CPL0_OFFSET: usize = offset_of!(ThreadContext, rsp_cpl0);
#[unsafe(no_mangle)]
pub static TC_CR3_OFFSET: usize = offset_of!(ThreadContext, cr3);
