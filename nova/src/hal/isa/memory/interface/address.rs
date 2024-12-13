use core::fmt::Debug;

pub trait VirtualAddress: Copy + Clone + Debug + PartialEq + Eq + PartialOrd + Ord + From<usize> + Into<usize> {
    fn from_ptr<T>(ptr: *const T) -> Self;
    fn from_mut<T>(ptr: *mut T) -> Self;
    fn into_ptr<T>(self) -> *const T;
    fn into_mut<T>(self) -> *mut T;
}

pub trait PhysicalAddress:
    Copy + Clone + Debug + PartialEq + Eq + PartialOrd + Ord + From<usize> + Into<usize>
{
    unsafe fn into_hhdm_ptr<T>(self) -> *const T;
    unsafe fn into_hhdm_mut<T>(self) -> *mut T;
}
