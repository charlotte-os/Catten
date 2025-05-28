use crate::llk::isa::interface::memory::address::{Address, PhysicalAddress, VirtualAddress};
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

impl Address for PAddr {
    const MAX: Self = PAddr(usize::MAX);
    const MIN: Self = PAddr(0);
    const NULL: Self = PAddr(0);

    fn is_aligned_to(&self, alignment: usize) -> bool {
        self.0 % alignment == 0
    }

    fn next_aligned_to(&self, alignment: usize) -> Self {
        PAddr::from((self.0 + alignment - 1) & !(alignment - 1))
    }

    fn is_valid(value: usize) -> bool {
        value & ((1 << 48) - 1) == value
    }

    fn is_null(&self) -> bool {
        self.0 == 0
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
