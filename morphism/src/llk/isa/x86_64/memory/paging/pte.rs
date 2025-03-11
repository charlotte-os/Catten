//! # Page Table Entry

use crate::llk::isa::x86_64::memory::address::paddr::PAddr;

/// PTE component indexes and masks
const PRESENT_BIT_INDEX: u64 = 0;
const WRITABLE_BIT_INDEX: u64 = 1;
const USER_ACCESSIBLE_BIT_INDEX: u64 = 2;
const PAT_INDEX_BITS_START: u64 = 3;
const PAT_INDEX_MASK: u64 = 0b11 << PAT_INDEX_BITS_START;
const ACCESSED_BIT_INDEX: u64 = 5;
const DIRTY_BIT_INDEX: u64 = 6;
const PAGE_SIZE_BIT_INDEX: u64 = 7;
const GLOBAL_BIT_INDEX: u64 = 8;
const FRAME_ADDR_START: u64 = 12;
const FRAME_ADDR_MASK: u64 = 0xfffffffffffff000;
const EXECUTE_DISABLE_BIT_INDEX: u64 = 63;

/// The page table entry structure
#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub fn new(
        present: bool,
        writable: bool,
        user_accessible: bool,
        pat_index_bits: u64,
        global: bool,
        frame_addr: PAddr,
    ) -> Self {
        let mut pte = Self(0);
        pte.set_present(present)
            .set_writable(writable)
            .set_user_accessible(user_accessible)
            .set_pat_index_bits(pat_index_bits)
            .set_global(global)
            .set_frame(frame_addr);
        pte
    }

    pub fn is_present(&self) -> bool {
        self.0 & (1 << PRESENT_BIT_INDEX) != 0
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        if present {
            self.0 |= 1 << PRESENT_BIT_INDEX;
        } else {
            self.0 &= !(1 << PRESENT_BIT_INDEX);
        }
        self
    }

    pub fn is_writable(&self) -> bool {
        self.0 & (1 << WRITABLE_BIT_INDEX) != 0
    }

    pub fn set_writable(&mut self, writable: bool) -> &mut Self {
        if writable {
            self.0 |= 1 << WRITABLE_BIT_INDEX;
        } else {
            self.0 &= !(1 << WRITABLE_BIT_INDEX);
        }
        self
    }

    pub fn is_user_accessible(&self) -> bool {
        self.0 & (1 << USER_ACCESSIBLE_BIT_INDEX) != 0
    }

    pub fn set_user_accessible(&mut self, user_accessible: bool) -> &mut Self {
        if user_accessible {
            self.0 |= 1 << USER_ACCESSIBLE_BIT_INDEX;
        } else {
            self.0 &= !(1 << USER_ACCESSIBLE_BIT_INDEX);
        }
        self
    }

    pub fn get_pat_index_bits(&self) -> u64 {
        (self.0 & PAT_INDEX_MASK) >> PAT_INDEX_BITS_START
    }

    pub fn set_pat_index_bits(&mut self, pat_index_bits: u64) -> &mut Self {
        self.0 = (self.0 & !PAT_INDEX_MASK) | ((pat_index_bits << PAT_INDEX_BITS_START) & PAT_INDEX_MASK);
        self
    }

    pub fn is_accessed(&self) -> bool {
        self.0 & (1 << ACCESSED_BIT_INDEX) != 0
    }

    pub fn set_accessed(&mut self, accessed: bool) -> &mut Self {
        if accessed {
            self.0 |= 1 << ACCESSED_BIT_INDEX;
        } else {
            self.0 &= !(1 << ACCESSED_BIT_INDEX);
        }
        self
    }

    pub fn is_dirty(&self) -> bool {
        self.0 & (1 << DIRTY_BIT_INDEX) != 0
    }

    pub fn set_dirty(&mut self, dirty: bool) -> &mut Self {
        if dirty {
            self.0 |= 1 << DIRTY_BIT_INDEX;
        } else {
            self.0 &= !(1 << DIRTY_BIT_INDEX);
        }
        self
    }

    pub fn get_page_size(&self) -> bool {
        self.0 & (1 << PAGE_SIZE_BIT_INDEX) != 0
    }

    pub fn set_page_size(&mut self, page_size: bool) -> &mut Self {
        if page_size {
            self.0 |= 1 << PAGE_SIZE_BIT_INDEX;
        } else {
            self.0 &= !(1 << PAGE_SIZE_BIT_INDEX);
        }
        self
    }

    pub fn is_global(&self) -> bool {
        self.0 & (1 << GLOBAL_BIT_INDEX) != 0
    }

    pub fn set_global(&mut self, global: bool) -> &mut Self {
        if global {
            self.0 |= 1 << GLOBAL_BIT_INDEX;
        } else {
            self.0 &= !(1 << GLOBAL_BIT_INDEX);
        }
        self
    }

    pub fn get_frame(&self) -> PAddr {
        PAddr::from((self.0 & FRAME_ADDR_MASK) as usize)
    }

    pub fn set_frame(&mut self, frame: PAddr) -> &mut Self {
        self.0 = (self.0 & !FRAME_ADDR_MASK) | ((<PAddr as Into<u64>>::into(frame)) & FRAME_ADDR_MASK);
        self
    }

    pub fn is_execute_disabled(&self) -> bool {
        self.0 & (1 << EXECUTE_DISABLE_BIT_INDEX) != 0
    }

    pub fn set_execute_disabled(&mut self, execute_disabled: bool) -> &mut Self {
        if execute_disabled {
            self.0 |= 1 << EXECUTE_DISABLE_BIT_INDEX;
        } else {
            self.0 &= !(1 << EXECUTE_DISABLE_BIT_INDEX);
        }
        self
    }
}
