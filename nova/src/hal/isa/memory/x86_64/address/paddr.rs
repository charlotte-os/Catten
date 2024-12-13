use crate::hal::isa::memory::interface::address::{PhysicalAddress, VirtualAddress};
use crate::memory::pmem::HHDM_BASE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PAddr {
    addr: usize,
}

impl PhysicalAddress for PAddr {
    unsafe fn into_hhdm_ptr<T>(self) -> *const T {
        (*HHDM_BASE).into_ptr::<T>()
    }

    unsafe fn into_hhdm_mut<T>(self) -> *mut T {
        (*HHDM_BASE).into_mut::<T>()
    }
}

impl<T> Into<*const T> for PAddr {
    fn into(self) -> *const T {
        unsafe { (*HHDM_BASE).into_ptr::<T>() }
    }
}

impl<T> Into<*mut T> for PAddr {
    fn into(self) -> *mut T {
        unsafe { (*HHDM_BASE).into_mut::<T>() }
    }
}

impl From<usize> for PAddr {
    fn from(value: usize) -> Self {
        PAddr {
            addr: value & *super::PADDR_MASK,
        }
    }
}

impl Into<usize> for PAddr {
    fn into(self) -> usize {
        self.addr
    }
}
