pub fn nearest_multiple_of<T>(value: T, multiple: T) -> T
where
    T: From<u64> + Into<u64> + Copy,
{
    let value: u64 = value.into();
    let multiple: u64 = multiple.into();
    if multiple == 0 {
        T::from(value)
    } else {
        T::from(((value + multiple / 2) / multiple) * multiple)
    }
}
