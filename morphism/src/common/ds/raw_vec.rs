use core::hint::{likely, unlikely};
use core::marker::PhantomData;

use crate::common::traits::TryClone;

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

impl<T> RawVec<T> {
    pub fn push(&mut self, val: T) -> Result<(), Error> {
        if unlikely(self.len == self.cap) {
            self.ptr = unsafe { (self.realloc)(self.ptr, self.cap, 2 * self.cap) }?;
            self.cap *= 2;
        }
        unsafe {
            *(*self.ptr.offset(self.len)) = val;
        }
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if likely(self.len > 0) {
            let val = unsafe { *(self.ptr.offset(self.len)) };
            self.len -= 1;
            Some(val)
        } else {
            None
        }
    }
}

impl<T: Clone> TryClone for RawVec<T> {
    type Error = Error;

    fn try_clone(&self) -> Result<RawVec<T>, Error> {
        //set up the structure of the new RawVector
        let mut new_rv = unsafe {
            RawVec::<T>::try_new(
                (self.realloc)(core::ptr::null_mut(), 0, self.cap),
                self.len,
                self.cap,
                self.realloc,
                self.dealloc,
            )?
        };
        // copy all the elements
        for i in 0..self.len {
            unsafe {
                *(new_rv.ptr.offset(i)) == (*(self.ptr.offset(i))).clone();
            }
        }
        Ok(new_rv)
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        unsafe {
            (self.dealloc)(self.ptr, self.cap);
        }
    }
}
