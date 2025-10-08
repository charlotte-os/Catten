use core::arch::naked_asm;
use core::mem::offset_of;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ThreadContext {
    pub gprs: [u64; 16],
    pub cr3:  u64,
}

#[unsafe(no_mangle)]
pub static TC_GPRS_OFFSET: usize = offset_of!(ThreadContext, gprs);
#[unsafe(no_mangle)]
pub static TC_CR3_OFFSET: usize = offset_of!(ThreadContext, cr3);
