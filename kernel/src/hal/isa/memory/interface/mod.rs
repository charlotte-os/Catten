pub trait AddrTypes {
    type VAddr: Clone 
        + Copy 
        + PartialEq 
        + Eq 
        + PartialOrd 
        + Ord 
        + From<usize>
        + Into<usize>
        + FromPtr
        + IntoPtr
        + LoadStore;
    type PAddr: Clone 
        + Copy 
        + PartialEq 
        + Eq 
        + PartialOrd 
        + Ord 
        + From<usize> 
        + Into<usize>
        + FromPtr
        + IntoPtr
        + LoadStore;
    type IoAddr: Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + LoadStore;
}

pub trait FromPtr {
    fn from_ptr<T>(ptr: *const T) -> Self;
    fn from_mut<T>(ptr: *mut T) -> Self;
}
pub trait IntoPtr {
    fn into_ptr<T>(self) -> *const T;
    fn into_mut<T>(self) -> *mut T;
}
pub unsafe trait LoadStore {
    unsafe fn load<T>(&self) -> T;
    unsafe fn store<T>(&self, value: T);
}
