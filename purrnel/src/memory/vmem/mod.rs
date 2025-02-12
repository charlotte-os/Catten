use crate::llk::isa::current_isa::memory::address::paddr::PAddr;
use crate::llk::isa::current_isa::memory::address::vaddr::VAddr;
use crate::llk::isa::interface::memory::MemoryInterface;
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
    pub fn is_user_accessible(&self) -> bool {
        match self {
            PageType::UserCode | PageType::UserData | PageType::UserRoData => true,
            _ => false,
        }
    }

    pub fn is_writable(&self) -> bool {
        match self {
            PageType::KernelData | PageType::UserData => true,
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
