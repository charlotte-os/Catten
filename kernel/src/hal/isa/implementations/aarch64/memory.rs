pub struct Memory;

impl crate::hal::isa::interfaces::memory::Memory for Memory {
    type VAddr = VAddr;
    type PAddr = PAddr;
    type IoAddr = VAddr;
}

struct VAddr {
    raw: usize
}

impl VAddr {
    pub fn get_raw(&self) -> usize {
        self.raw
    }
    pub fn as_ptr<T>(&self) -> *const T {
        self.raw as *const T
    }
    pub fn as_mut<T>(&self) -> *mut T {
        self.raw as *mut T
    }
}