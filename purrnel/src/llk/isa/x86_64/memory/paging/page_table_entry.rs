#[repr(transparent)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn present(&self) -> bool {
        self.0 & 1 != 0
    }

    pub fn set_present(&mut self, present: bool) {
        if present {
            self.0 |= 1;
        } else {
            self.0 &= !1;
        }
    }

    pub fn writable(&self) -> bool {
        self.0 & (1 << 1) != 0
    }

    pub fn set_writable(&mut self, writable: bool) {
        if writable {
            self.0 |= 1 << 1;
        } else {
            self.0 &= !(1 << 1);
        }
    }

    pub fn user_accessible(&self) -> bool {
        self.0 & (1 << 2) != 0
    }

    pub fn set_user_accessible(&mut self, user_accessible: bool) {
        if user_accessible {
            self.0 |= 1 << 2;
        } else {
            self.0 &= !(1 << 2);
        }
    }

    pub fn pat_index_bits(&self) -> u64 {
        (self.0 >> 3) & 0b11
    }

    pub fn set_pat_index_bits(&mut self, pat_index_bits: u64) {
        self.0 = (self.0 & !(0b11 << 3)) | ((pat_index_bits & 0b11) << 3);
    }

    pub fn accessed(&self) -> bool {
        self.0 & (1 << 5) != 0
    }
    pub fn set_accessed(&mut self, accessed: bool) {
        if accessed {
            self.0 |= 1 << 5;
        } else {
            self.0 &= !(1 << 5);
        }
    }
    pub fn dirty(&self) -> bool {
        self.0 & (1 << 6) != 0
    }
    pub fn set_dirty(&mut self, dirty: bool) {
        if dirty {
            self.0 |= 1 << 6;
        } else {
            self.0 &= !(1 << 6);
        }
    }
    pub fn page_size(&self) -> bool {
        self.0 & (1 << 7) != 0
    }
    pub fn set_page_size(&mut self, page_size: bool) {
        if page_size {
            self.0 |= 1 << 7;
        } else {
            self.0 &= !(1 << 7);
        }
    }
    pub fn global(&self) -> bool {
        self.0 & (1 << 8) != 0
    }
    pub fn set_global(&mut self, global: bool) {
        if global {
            self.0 |= 1 << 8;
        } else {
            self.0 &= !(1 << 8);
        }
    }
    pub fn frame(&self) -> u64 {
        self.0 >> 12
    }
    pub fn set_frame(&mut self, frame: u64) {
        self.0 = (self.0 & 0xFFF) | (frame << 12);
    }
}