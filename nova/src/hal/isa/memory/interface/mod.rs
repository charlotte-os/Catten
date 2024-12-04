pub trait AddrTypes {
    type VAddr: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + From<usize> + Into<usize> + FromPtr + IntoPtr;
    type PAddr: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + From<usize> + Into<usize> + IntoPtr;
}

pub trait FromPtr {
    fn from_ptr<T>(ptr: *const T) -> Self;
    fn from_mut<T>(ptr: *mut T) -> Self;
}
pub trait IntoPtr {
    fn into_ptr<T>(self) -> *const T;
    fn into_mut<T>(self) -> *mut T;
}
