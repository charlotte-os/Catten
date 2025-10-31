use core::mem::offset_of;

use crate::memory::AddressSpaceId;

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct ThreadContext {
    pub cr3: u64,
    pub rsp: u64,
    pub rip: u64,
}

impl ThreadContext {
    pub fn get_asid(&self) -> AddressSpaceId {
        self.cr3 as AddressSpaceId
    }
}

#[unsafe(no_mangle)]
pub static TC_CR3_OFFSET: usize = offset_of!(ThreadContext, cr3);
#[unsafe(no_mangle)]
pub static TC_RSP_OFFSET: usize = offset_of!(ThreadContext, rsp);
