use alloc::boxed::Box;
use alloc::vec::Vec;

pub fn make_boxed_slice<T, F: Fn() -> T>(count: usize, generator: F) -> Box<[T]> {
    let mut v = Vec::<T>::new();
    for _ in 0..count {
        v.push(generator())
    }
    v.into_boxed_slice()
}
