use core::mem::offset_of;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ThreadState {
    pub gprs: [u64; 16],
    pub cr3:  u64,
}

#[unsafe(no_mangle)]
pub static TS_GPRS_OFFSET: usize = offset_of!(ThreadState, gprs);
#[unsafe(no_mangle)]
pub static TS_CR3_OFFSET: usize = offset_of!(ThreadState, cr3);
