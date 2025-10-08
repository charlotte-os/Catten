use crate::isa::aarch64::system_info::CpuInfo;
use crate::isa::interface::memory::address::{Address, VirtualAddress};
use crate::isa::interface::system_info::CpuInfoIfce;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VAddr(usize);

impl Address for VAddr {
    const MAX: Self = VAddr(usize::MAX);
    const MIN: Self = VAddr(0);
    const NULL: Self = VAddr(0);

    fn is_aligned_to(&self, alignment: usize) -> bool {
        self.0 % alignment == 0
    }

    fn is_valid(value: usize) -> bool {
        value & ((1 << CpuInfo::get_paddr_sig_bits()) - 1) == value
    }

    fn is_null(&self) -> bool {
        self.0 == 0
    }

    fn next_aligned_to(&self, alignment: usize) -> Self {
        VAddr((self.0 + alignment - 1) & !(alignment - 1))
    }
}

impl VirtualAddress for VAddr {
    fn from_ptr<T>(ptr: *const T) -> Self {
        VAddr(ptr as usize)
    }

    fn from_mut<T>(ptr: *mut T) -> Self {
        VAddr(ptr as usize)
    }

    fn into_ptr<T>(self) -> *const T {
        self.0 as *const T
    }

    fn into_mut<T>(self) -> *mut T {
        self.0 as *mut T
    }
}

impl From<usize> for VAddr {
    fn from(val: usize) -> Self {
        VAddr(val)
    }
}

impl Into<usize> for VAddr {
    fn into(self) -> usize {
        self.0
    }
}
