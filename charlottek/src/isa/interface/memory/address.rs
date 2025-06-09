use core::fmt::Debug;

pub trait Address: Copy + Clone + Debug + PartialEq + Eq + PartialOrd + Ord + From<usize> + Into<usize> {
    const MIN: Self;
    const MAX: Self;
    const NULL: Self;

    fn is_aligned_to(&self, alignment: usize) -> bool;
    fn next_aligned_to(&self, alignment: usize) -> Self;
    fn is_valid(value: usize) -> bool;
    fn is_null(&self) -> bool;
}

pub trait VirtualAddress: Address {
    fn from_ptr<T>(ptr: *const T) -> Self;
    fn from_mut<T>(ptr: *mut T) -> Self;
    fn into_ptr<T>(self) -> *const T;
    fn into_mut<T>(self) -> *mut T;
}

pub trait PhysicalAddress: Address {
    unsafe fn into_hhdm_ptr<T>(self) -> *const T;
    unsafe fn into_hhdm_mut<T>(self) -> *mut T;
}
