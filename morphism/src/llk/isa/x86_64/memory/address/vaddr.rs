use crate::llk::isa::interface::memory::address::VirtualAddress;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VAddr {
    raw: usize,
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
            let is_negative = (value & ((1 << *super::VADDR_SIG_BITS) - 1)) != 0;
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
