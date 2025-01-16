use crate::hal::isa::interface::memory::address::{PhysicalAddress, VirtualAddress};
use crate::memory::pmem::HHDM_BASE;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PAddr(usize);

impl From<usize> for PAddr {
    fn from(val: usize) -> Self {
        PAddr(val)
    }
}

impl Into<usize> for PAddr {
    fn into(self) -> usize {
        self.0
    }
}

impl PhysicalAddress for PAddr {
    unsafe fn into_hhdm_ptr<T>(self) -> *const T {
        (*HHDM_BASE).into_ptr::<T>().byte_offset(self.0 as isize)
    }

    unsafe fn into_hhdm_mut<T>(self) -> *mut T {
        (*HHDM_BASE).into_mut::<T>().byte_offset(self.0 as isize)
    }
}

impl core::ops::Add<isize> for PAddr {
    type Output = PAddr;
    fn add(self, rhs: isize) -> Self::Output {
        PAddr::from(self.0.wrapping_add(rhs as usize))
    }
}
