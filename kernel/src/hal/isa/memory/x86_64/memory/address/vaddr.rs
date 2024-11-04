#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct VAddr {
    raw: usize
}

impl From<usize> for VAddr {
    fn from(value: usize) -> Self {
        VAddr {
            raw: value
        }
    }
}

impl Into<usize> for VAddr {
    fn into(self) -> usize {
        self.raw
    }
}

impl memory::FromPtr for VAddr {
    fn from_ptr<T>(ptr: *const T) -> Self {
        VAddr {
            raw: ptr as usize
        }
    }
    fn from_mut<T>(ptr: *mut T) -> Self {
        VAddr {
            raw: ptr as usize
        }
    }
}

impl memory::IntoPtr for VAddr {
    fn into_ptr<T>(self) -> *const T {
        self.raw as *const T
    }
    fn into_mut<T>(self) -> *mut T {
        self.raw as *mut T
    }
}

impl memory::LoadStore for VAddr {
    unsafe fn load<T>(&self) -> T {
        *(self.raw as *const T)
    }
    unsafe fn store<T>(&self, value: T) {
        core::ptr::write_volatile(self.raw as *mut T, value);
    }
}