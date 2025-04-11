pub use crate::llk::isa::current_isa::memory::address::paddr::PAddr;
pub use crate::llk::isa::current_isa::memory::address::vaddr::VAddr;
use crate::llk::isa::interface::memory::MemoryInterface;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageType {
    KernelCode,   //read, execute
    KernelData,   //read, write
    KernelRoData, //read only
    UserCode,     //user, read, execute
    UserData,     //user, read, write
    UserRoData,   //user, read only
    Mmio,         //read, write, no caching
    Framebuffer,  //read, write, write combining
}

impl PageType {
    pub fn new(is_user_accessible: bool, is_writeable: bool, is_no_execute: bool) -> Self {
        match (is_user_accessible, is_writeable, is_no_execute) {
            (false, false, false) => PageType::KernelCode,
            (false, true, false) => PageType::KernelData,
            (false, false, true) => PageType::KernelRoData,
            (true, false, false) => PageType::UserCode,
            (true, true, false) => PageType::UserData,
            (true, false, true) => PageType::UserRoData,
            (_, _, _) => panic!("Invalid page type"),
        }
    }

    pub fn is_user_accessible(&self) -> bool {
        match self {
            PageType::UserCode | PageType::UserData | PageType::UserRoData => true,
            _ => false,
        }
    }

    pub fn is_writable(&self) -> bool {
        match self {
            PageType::KernelData | PageType::UserData | PageType::Mmio | PageType::Framebuffer => {
                true
            }
            _ => false,
        }
    }

    pub fn is_no_execute(&self) -> bool {
        match self {
            PageType::KernelCode | PageType::UserCode => false,
            _ => true,
        }
    }
}

pub struct MemoryMapping {
    pub vaddr: VAddr,
    pub paddr: PAddr,
    pub page_type: PageType,
}
