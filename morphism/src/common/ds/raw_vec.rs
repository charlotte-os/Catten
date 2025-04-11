use core::hint::{likely, unlikely};
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

use crate::common::traits::TryClone;

pub enum Error {
    InvalidArgCombination,
    ImproperlyAlignedPointer,
    IndexOutOfRange,
}

/// RawVec is a low-level vector implementation that takes function pointers for reallocation and
/// deallocation. It exists to provide a Vector implementation for use in the kernel allocator
/// without running into circular dependencies.
pub struct RawVec<T: Sized + Clone> {
    ptr: *mut T,
    len: usize,
    cap: usize,
    _marker: PhantomData<T>,
    realloc: unsafe fn(ptr: *mut T, curr_cap: usize, new_cap: usize) -> Result<*mut T, Error>,
    dealloc: unsafe fn(ptr: *mut T, cap: usize),
}

impl<T: Sized + Clone> RawVec<T> {
    pub unsafe fn try_new(
        ptr: *mut T,
        len: usize,
        cap: usize,
        realloc: unsafe fn(ptr: *mut T, curr_cap: usize, new_cap: usize) -> Result<*mut T, Error>,
        dealloc: unsafe fn(ptr: *mut T, cap: usize),
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

impl<T: Sized + Clone + Copy> RawVec<T> {
    fn double_capacity(&mut self) -> Result<(), Error> {
        self.ptr = unsafe { (self.realloc)(self.ptr, self.cap, 2 * self.cap) }?;
        self.cap *= 2;
        Ok(())
    }

    pub fn push(&mut self, val: T) -> Result<(), Error> {
        if unlikely(self.len == self.cap) {
            self.double_capacity()?
        }
        unsafe {
            *(self.ptr.offset(self.len as isize)) = val;
        }
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if likely(self.len > 0) {
            let val = unsafe { self.ptr.offset(self.len as isize).read() };
            self.len -= 1;
            Some(val)
        } else {
            None
        }
    }

    pub fn insert(&mut self, index: isize, val: T) -> Result<(), Error> {
        if index > self.len as isize || index < 0isize {
            Err(Error::IndexOutOfRange)
        } else if index == self.len as isize {
            self.push(val)
        } else {
            // ensure sufficient capacity is reserved
            if self.cap == self.len {
                self.double_capacity()?
            }
            // shift each element starting at the given index up one slot in the vector
            for i in (index..self.len as isize).rev() {
                unsafe {
                    let (curr, next) = (self.ptr.offset(i), self.ptr.offset(i + 1));
                    next.write(curr.read())
                }
            }
            self[index] = val;
            Ok(())
        }
    }
}

impl<T: Sized + Clone> Index<isize> for RawVec<T>
where
    T: Sized + Copy,
{
    type Output = T;

    fn index(&self, index: isize) -> &T {
        unsafe { &*(self.ptr.offset(index)) }
    }
}

impl<T: Sized + Clone + Copy> IndexMut<isize> for RawVec<T> {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        unsafe { &mut *(self.ptr.offset(index)) }
    }
}

impl<T: Sized + Clone> TryClone for RawVec<T> {
    type Error = Error;

    fn try_clone(&self) -> Result<RawVec<T>, Error> {
        //set up the structure of the new RawVector
        let mut new_rv = unsafe {
            RawVec::<T>::try_new(
                (self.realloc)(core::ptr::null_mut(), 0, self.cap)?,
                self.len,
                self.cap,
                self.realloc,
                self.dealloc,
            )?
        };
        // copy all the elements
        for i in 0..self.len as isize {
            unsafe {
                *(new_rv.ptr.offset(i)) = (*(self.ptr.offset(i))).clone();
            }
        }
        Ok(new_rv)
    }
}

impl<T: Sized + Clone> Drop for RawVec<T> {
    fn drop(&mut self) {
        unsafe {
            (self.dealloc)(self.ptr, self.cap);
        }
    }
}
