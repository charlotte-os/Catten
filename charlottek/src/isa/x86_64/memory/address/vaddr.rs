use core::iter::Step;
use core::ops::{Add, Sub};

use crate::isa::interface::memory::address::{Address, VirtualAddress};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VAddr {
    raw: usize,
}

impl core::fmt::Debug for VAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "VAddr({:#x})", self.raw)
    }
}

/// VAddr component indexes and masks
const PAGE_TABLE_INDEX_MASK: usize = 0x1ff;
const PML4_INDEX_SHIFT: usize = 39;
const PML4_INDEX_MASK: usize = PAGE_TABLE_INDEX_MASK << PML4_INDEX_SHIFT;
const PDPT_INDEX_SHIFT: usize = 30;
const PDPT_INDEX_MASK: usize = PAGE_TABLE_INDEX_MASK << PDPT_INDEX_SHIFT;
const PD_INDEX_SHIFT: usize = 21;
const PD_INDEX_MASK: usize = PAGE_TABLE_INDEX_MASK << PD_INDEX_SHIFT;
const PT_INDEX_SHIFT: usize = 12;
const PT_INDEX_MASK: usize = PAGE_TABLE_INDEX_MASK << PT_INDEX_SHIFT;
const OFFSET_MASK: usize = 0xfff;

impl VAddr {
    /// Convenience functions to get the index for each level of the page table hierarchy

    pub fn pml4_index(&self) -> usize {
        (self.raw & PML4_INDEX_MASK) >> PML4_INDEX_SHIFT
    }

    pub fn pdpt_index(&self) -> usize {
        (self.raw & PDPT_INDEX_MASK) >> PDPT_INDEX_SHIFT
    }

    pub fn pd_index(&self) -> usize {
        (self.raw & PD_INDEX_MASK) >> PD_INDEX_SHIFT
    }

    pub fn pt_index(&self) -> usize {
        (self.raw & PT_INDEX_MASK) >> PT_INDEX_SHIFT
    }

    pub fn page_offset(&self) -> usize {
        self.raw & OFFSET_MASK
    }

    /// Safety: The address must be in canonical form
    pub const unsafe fn from_raw_unchecked(raw: usize) -> Self {
        VAddr { raw }
    }
}

impl Address for VAddr {
    const MAX: Self = VAddr { raw: usize::MAX };
    const MIN: Self = VAddr { raw: 0 };
    const NULL: Self = VAddr { raw: 0 };

    fn is_aligned_to(&self, alignment: usize) -> bool {
        self.raw % alignment == 0
    }

    fn next_aligned_to(&self, alignment: usize) -> Self {
        let mask = alignment - 1;
        let aligned = (<VAddr as Into<usize>>::into(*self) + mask) & !mask;
        VAddr::from(aligned)
    }

    fn is_valid(value: usize) -> bool {
        value != 0
    }

    fn is_null(&self) -> bool {
        self.raw == 0
    }
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
        crate::logln!("VADDR_SIG_BITS = {}", (*super::VADDR_SIG_BITS));
        let mask = (1 << *super::VADDR_SIG_BITS) - 1;
        let sign_extended = if value & (1 << (*super::VADDR_SIG_BITS - 1)) != 0 {
            value | (!mask)
        } else {
            value & mask
        };
        VAddr { raw: sign_extended }
    }
}

impl Into<usize> for VAddr {
    fn into(self) -> usize {
        self.raw
    }
}

impl Sub for VAddr {
    type Output = isize;

    fn sub(self, other: Self) -> Self::Output {
        self.raw as isize - other.raw as isize
    }
}

impl Add<isize> for VAddr {
    type Output = VAddr;

    fn add(self, other: isize) -> Self::Output {
        VAddr {
            raw: (self.raw as isize + other) as usize,
        }
    }
}

impl Step for VAddr {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        if start > end {
            (0, None)
        } else {
            (end.raw - start.raw, Some(end.raw - start.raw))
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        if start.raw.saturating_add(count) < usize::MAX {
            Some(VAddr { raw: start.raw + count })
        } else {
            None
        }
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if start.raw.saturating_sub(count) > usize::MIN {
            Some(VAddr { raw: start.raw - count })
        } else {
            None
        }
    }
}
