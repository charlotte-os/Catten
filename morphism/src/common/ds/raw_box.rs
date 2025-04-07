#[derive(Debug)]
pub struct RawBox<T> {
    ptr: *mut T,
    release: fn(*mut T),
}

impl<T> RawBox<T> {
    pub fn new(addr: *mut T, releaser: fn(*mut T)) -> Self {
        RawBox {
            ptr: addr,
            release: releaser,
        }
    }
}

impl<T> Drop for RawBox<T> {
    fn drop(&mut self) {
        //call the function by pointer
        (self.release)(self.ptr)
    }
}

impl<T> core::ops::Deref for RawBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> core::ops::DerefMut for RawBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T> AsRef<T> for RawBox<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> AsMut<T> for RawBox<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T> core::fmt::Pointer for RawBox<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Pointer::fmt(&self.ptr, f)
    }
}
