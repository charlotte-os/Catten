pub use crate::isa::current_isa::memory::address::paddr::PAddr;
pub use crate::isa::current_isa::memory::address::vaddr::VAddr;

pub type AddressSpaceId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidPageAttributes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageType {
    KernelCode,         //read, execute
    KernelData,         //read, write
    KernelRoData,       //read only
    UserCode,           //user, read, execute
    UserData,           //user, read, write
    UserRoData,         //user, read only
    Mmio,               //read, write, no caching
    DirectMemoryAccess, //read, write, no caching
    Framebuffer,        //read, write, write combining
}

impl PageType {
    pub fn is_user_accessible(&self) -> bool {
        match *self {
            PageType::UserCode | PageType::UserData | PageType::UserRoData => true,
            _ => false,
        }
    }

    pub fn is_writable(&self) -> bool {
        match *self {
            PageType::KernelData
            | PageType::UserData
            | PageType::Mmio
            | PageType::DirectMemoryAccess
            | PageType::Framebuffer => true,
            _ => false,
        }
    }

    pub fn is_no_execute(&self) -> bool {
        match *self {
            PageType::KernelCode | PageType::UserCode => false,
            _ => true,
        }
    }

    pub fn is_uncacheable(&self) -> bool {
        match *self {
            PageType::Mmio | PageType::DirectMemoryAccess | PageType::Framebuffer => true,
            _ => false,
        }
    }

    pub fn should_combine_writes(&self) -> bool {
        if *self == PageType::Framebuffer {
            true
        } else {
            false
        }
    }
}

pub struct MemoryMapping {
    pub vaddr: VAddr,
    pub paddr: PAddr,
    pub page_type: PageType,
}
