use core::ops::Add;

use crate::isa::interface::memory::address::{Address, PhysicalAddress, VirtualAddress};
use crate::memory::pmem::HHDM_BASE;

#[derive(Debug, Clone, Copy)]
pub enum PAddrError {
    OutOfCpuSupportedRange(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PAddr {
    addr: usize,
}

impl Address for PAddr {
    const MAX: Self = PAddr { addr: usize::MAX };
    const MIN: Self = PAddr { addr: 0 };
    const NULL: Self = PAddr { addr: 0 };

    fn is_aligned_to(&self, alignment: usize) -> bool {
        self.addr % alignment == 0
    }

    fn is_valid(value: usize) -> bool {
        value & *super::PADDR_MASK == value
    }

    fn is_null(&self) -> bool {
        self.addr == 0
    }

    fn next_aligned_to(&self, alignment: usize) -> Self {
        unsafe { PAddr::from_unchecked((self.addr + alignment - 1) & !(alignment - 1)) }
    }

    unsafe fn from_unchecked(addr: usize) -> Self {
        PAddr { addr }
    }
}

impl PhysicalAddress for PAddr {
    unsafe fn into_hhdm_ptr<T>(self) -> *const T {
        unsafe { (*HHDM_BASE).into_ptr::<T>().byte_offset(self.addr as isize) }
    }

    unsafe fn into_hhdm_mut<T>(self) -> *mut T {
        unsafe { (*HHDM_BASE).into_mut::<T>().byte_offset(self.addr as isize) }
    }
}

impl<T> Into<*const T> for PAddr {
    fn into(self) -> *const T {
        unsafe { (*HHDM_BASE).into_ptr::<T>().byte_offset(self.addr as isize) }
    }
}

impl<T> Into<*mut T> for PAddr {
    fn into(self) -> *mut T {
        unsafe { (*HHDM_BASE).into_mut::<T>().byte_offset(self.addr as isize) }
    }
}

impl TryFrom<usize> for PAddr {
    type Error = PAddrError;

    fn try_from(value: usize) -> Result<Self, PAddrError> {
        if value & !*super::PADDR_MASK != 0 {
            Err(PAddrError::OutOfCpuSupportedRange(value))
        } else {
            Ok(PAddr { addr: value })
        }
    }
}

impl Into<usize> for PAddr {
    fn into(self) -> usize {
        self.addr
    }
}

impl From<u64> for PAddr {
    fn from(value: u64) -> Self {
        PAddr {
            addr: value as usize & *super::PADDR_MASK,
        }
    }
}

impl Into<u64> for PAddr {
    fn into(self) -> u64 {
        self.addr as u64
    }
}

impl Add<isize> for PAddr {
    type Output = PAddr;

    fn add(self, rhs: isize) -> Self::Output {
        PAddr::try_from(self.addr.wrapping_add(rhs as usize)).unwrap()
    }
}
