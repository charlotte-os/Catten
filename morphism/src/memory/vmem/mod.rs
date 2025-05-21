pub use crate::llk::isa::current_isa::memory::address::paddr::PAddr;
pub use crate::llk::isa::current_isa::memory::address::vaddr::VAddr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidPageAttributes,
}

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
    pub fn try_new(
        is_user_accessible: bool,
        is_writeable: bool,
        is_no_execute: bool,
        is_uncacheable: bool,
        should_combine_writes: bool,
    ) -> Result<Self, Error> {
        match (
            is_user_accessible,
            is_writeable,
            is_no_execute,
            is_uncacheable,
            should_combine_writes,
        ) {
            (false, true, true, true, false) => Ok(PageType::Mmio),
            (false, true, true, true, true) => Ok(PageType::Framebuffer),
            (false, false, false, _, _) => Ok(PageType::KernelCode),
            (false, true, true, _, _) => Ok(PageType::KernelData),
            (false, false, true, _, _) => Ok(PageType::KernelRoData),
            (true, false, false, _, _) => Ok(PageType::UserCode),
            (true, true, true, _, _) => Ok(PageType::UserData),
            (true, false, true, _, _) => Ok(PageType::UserRoData),
            (_, _, _, _, _) => Err(Error::InvalidPageAttributes),
        }
    }

    pub fn is_user_accessible(&self) -> bool {
        match *self {
            PageType::UserCode | PageType::UserData | PageType::UserRoData => true,
            _ => false,
        }
    }

    pub fn is_writable(&self) -> bool {
        match *self {
            PageType::KernelData | PageType::UserData | PageType::Mmio | PageType::Framebuffer => {
                true
            }
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
            PageType::Mmio | PageType::Framebuffer => true,
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
