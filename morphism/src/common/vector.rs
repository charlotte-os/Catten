use core::alloc::{AllocError, Allocator, Layout, LayoutError};
use core::mem::transmute;
use core::ops::{Index, IndexMut, Neg};

use crate::memory::allocator::KERNEL_ALLOCATOR;

#[derive(Debug)]
pub enum VecError {
    AllocError(AllocError),
    Empty,
    LayoutError(LayoutError),
}

impl From<LayoutError> for VecError {
    fn from(err: LayoutError) -> Self {
        VecError::LayoutError(err)
    }
}

impl From<AllocError> for VecError {
    fn from(err: AllocError) -> Self {
        VecError::AllocError(err)
    }
}

#[derive(Debug)]
pub struct Vec<T> {
    data: *mut T,
    len:  usize,
    cap:  usize,
}

impl<T> Vec<T> {
    pub const fn new() -> Self {
        Self {
            data: core::ptr::null_mut(),
            len:  0,
            cap:  0,
        }
    }

    fn resize(&mut self, new_cap: usize) -> Result<(), VecError> {
        let new_arr = KERNEL_ALLOCATOR
            .allocate(Layout::from_size_align(
                core::mem::size_of::<T>() * new_cap,
                core::mem::align_of::<T>(),
            )?)?
            .as_mut_ptr() as *mut T;
        if !self.data.is_null() {
            unsafe {
                core::ptr::copy_nonoverlapping(self.data, new_arr, self.len);
                KERNEL_ALLOCATOR.deallocate(
                    transmute(self.data),
                    Layout::from_size_align(core::mem::size_of::<T>() * self.cap, core::mem::align_of::<T>())?,
                );
            }
        }
        self.data = new_arr;
        self.cap = new_cap;
        Ok(())
    }

    pub fn push(&mut self, value: T) -> Result<(), VecError> {
        if self.len == self.cap {
            self.resize(self.cap * 2)?;
        }
        unsafe {
            core::ptr::write(self.data.add(self.len), value);
        }
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T, VecError> {
        if self.len == 0 {
            Err(VecError::Empty)
        } else {
            self.len -= 1;
            let value = unsafe { core::ptr::read(self.data.add(self.len)) };
            Ok(value)
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Index out of bounds");
        }
        unsafe { self.data.offset(index as isize).as_ref_unchecked() }
    }
}

impl<T> IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("Index out of bounds");
        }
        unsafe { self.data.offset(index as isize).as_mut_unchecked() }
    }
}

pub struct VecIter<T: Clone> {
    vec: *const Vec<T>,
    index: usize,
}
impl<'a, T: Clone> Iterator for VecIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < unsafe { (*self.vec).len } {
            let value = unsafe { (*self.vec).data.add(self.index).read() };
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}
impl<T: Clone> IntoIterator for Vec<T> {
    type IntoIter = VecIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        VecIter { vec: &self, index: 0 }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                KERNEL_ALLOCATOR.deallocate(
                    transmute(self.data),
                    Layout::from_size_align(core::mem::size_of::<T>() * self.cap, core::mem::align_of::<T>()).unwrap(),
                );
            }
        }
    }
}
