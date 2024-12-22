use crate::hal::isa::interface::memory::address::VirtualAddress;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VAddr {
    raw: usize,
}

impl VirtualAddress for VAddr {
    fn from_ptr<T>(ptr: *const T) -> Self {
        VAddr { raw: ptr as usize }
    }

    fn from_mut<T>(ptr: *mut T) -> Self {
        VAddr { raw: ptr as usize }
    }

    fn into_ptr<T>(self) -> *const T {
        self.raw as *const T
    }

    fn into_mut<T>(self) -> *mut T {
        self.raw as *mut T
    }
}

impl From<usize> for VAddr {
    fn from(value: usize) -> Self {
        let corrected = {
            let is_negative = (value & (1 << (*super::VADDR_SIG_BITS - 1))) != 0;
            if is_negative {
                value | !(*super::VADDR_MASK)
            } else {
                value & *super::VADDR_MASK
            }
        };
        VAddr { raw: corrected }
    }
}

impl Into<usize> for VAddr {
    fn into(self) -> usize {
        self.raw
    }
}
