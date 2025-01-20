use core::cell::UnsafeCell;

/// A wrapper around `UnsafeCell` that is `Send`.
///
/// This is useful for when you need to store a value in a `static` variable, but the value is not `Send`.
/// Safety: The developer must ensure that the contained value is not used to access memory that is aliased elsewhere.
pub struct SendUnsafeCell<T> {
    value: UnsafeCell<T>,
}

impl<T> SendUnsafeCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub unsafe fn get(&self) -> &T {
        unsafe { &*self.value.get() }
    }

    pub unsafe fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.value.get() }
    }
}

unsafe impl<T> Send for SendUnsafeCell<T> {}
