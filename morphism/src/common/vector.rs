use core::Option;
use core::alloc::{Layout, LayoutError};
use core::ptr::NonNull;

pub enum VectorError {
    LayoutError(LayoutError),
}

impl From<LayoutError> for VectorError {
    fn from(err: LayoutError) -> Self {
        VectorError::LayoutError(err)
    }
}

pub struct Vector<T> {
    data: Option<NonNull<T>>,
    len:  usize,
    cap:  usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self {
            data: None,
            len:  0,
            cap:  0,
        }
    }

    fn resize(&mut self, new_cap: usize) -> Result<(), VectorError> {
        let new_arr = KERNEL_ALLOCATOR.allocate(Layout::from_size_align(
            core::mem::size_of::<T>() * new_cap,
            core::mem::align_of::<T>(),
        )?)?;
        unsafe {
            core::ptr::copy_nonoverlapping(self.data, new_data, self.len);
            core::alloc::dealloc(self.data, core::alloc::Layout::array::<T>(self.cap).unwrap());
        }
        KERNEL_ALLOCATOR.deallocate(
            self.data.unwrap().cast(),
            Layout::from_size_align(core::mem::size_of::<T>() * self.cap, core::mem::align_of::<T>())?,
        );
        self.data = new_arr;
        self.cap = new_cap;
    }

    pub fn push(&mut self, value: T) -> Result<(), VectorError> {
        if self.len == self.cap {
            self.resize(self.cap * 2)?;
        }
        unsafe {
            core::ptr::write(self.data.unwrap().as_ptr().add(self.len), value);
        }
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<T, VectorError> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let value = unsafe { core::ptr::read(self.data.unwrap().as_ptr().add(self.len)) };
        Some(value)
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if let Some(data) = self.data {
            KERNEL_ALLOCATOR.deallocate(
                data.cast(),
                Layout::from_size_align(core::mem::size_of::<T>() * self.cap, core::mem::align_of::<T>())?,
            );
        }
    }
}
