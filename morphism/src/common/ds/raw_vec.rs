use core::marker::PhantomData;

pub enum Error {
    InvalidArgCombination,
    ImproperlyAlignedPointer,
}

/// RawVec is a low-level vector implementation that takes function pointers for reallocation and
/// deallocation. It exists to provide a Vector implementation for use in the kernel allocator
/// without running into circular dependencies.
pub struct RawVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
    realloc: unsafe fn(ptr: *mut T, curr_cap: usize, new_cap: usize) -> Result<*mut T, Error>,
    dealloc: unsafe fn(ptr: *mut T, cap: usize),
}

impl<T> RawVec<T> {
    pub unsafe fn try_new(
        ptr: *mut T,
        len: usize,
        cap: usize,
        realloc: fn(ptr: *mut T, curr_cap: usize, new_cap: usize) -> Result<*mut T, Error>,
        dealloc: fn(ptr: *mut T, cap: usize),
    ) -> Result<Self, Error> {
        // validate arguments then return the RawVec
        if cap > 0 && ptr.is_null() {
            Err(Error::InvalidArgCombination)
        } else if len > cap {
            Err(Error::InvalidArgCombination)
        } else if cap == 0 && !ptr.is_null() {
            Err(Error::InvalidArgCombination)
        } else if !ptr.is_aligned() {
            Err(Error::ImproperlyAlignedPointer)
        } else {
            Ok(RawVec {
                ptr: ptr,
                len: len,
                cap: cap,
                _marker: PhantomData,
                realloc: realloc,
                dealloc: dealloc,
            })
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        unsafe {
            (self.dealloc)(self.ptr, self.cap);
        }
    }
}
