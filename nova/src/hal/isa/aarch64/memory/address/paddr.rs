use crate::hal::isa::memory::interface::address::PhysicalAddress;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PAddr(usize);

impl PhysicalAddress for PAddr {
    unsafe fn into_hhdm_ptr<T>(self) -> *const T {
        (*HHDM_BASE).into_ptr::<T>().byte_offset(self.0 as isize)
    }

    unsafe fn into_hhdm_mut<T>(self) -> *mut T {
        (*HHDM_BASE).into_ptr::<T>().byte_offset(self.0 as isize)
    }
}

pub impl From<usize> for VAddr {
    fn from(val: usize) -> Self {
        VAddr(val)
    }
}

pub impl Into<usize> for VAddr {
    fn into(self) -> usize {
        self.0
    }
}
