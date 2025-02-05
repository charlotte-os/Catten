use crate::llk::isa::interface::memory::MemoryInterface;

pub enum PageType {
    Memory,
    Mmio,
    Framebuffer,
}

pub struct MemoryMapping {
    pub vaddr: usize,
    pub paddr: usize,
    pub page_type: PageType,
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}
