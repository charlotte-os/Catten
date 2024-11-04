use crate::hal::isa::memory::interface;
use crate::hal::bootinfo::HHDM_BASE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PAddr {
    addr: usize
}

impl interface::IntoPtr for PAddr {
    fn into_ptr<T>(self) -> *const T {
        unsafe {
            (*HHDM_BASE).into_ptr::<T>().byte_offset(self.addr as isize)
        }
    }
    fn into_mut<T>(self) -> *mut T {
        unsafe {
            (*HHDM_BASE).into_mut::<T>().byte_offset(self.addr as isize)
        }
    }
}

impl From<usize> for PAddr {
    fn from(value: usize) -> Self {
        PAddr {
            addr: value & *super::PADDR_MASK
        }
    }
}

impl Into<usize> for PAddr {
    fn into(self) -> usize {
        self.addr
    }
}