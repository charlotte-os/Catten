//! Utilities for Working with Memory Sizes

pub const fn kibibytes(n: usize) -> usize {
    n * 1024
}
pub const fn mebibytes(n: usize) -> usize {
    n * 1024 * 1024
}
pub const fn gibibytes(n: usize) -> usize {
    n * 1024 * 1024 * 1024
}
pub const fn tebibytes(n: usize) -> usize {
    n * 1024 * 1024 * 1024 * 1024
}
pub const fn pebibytes(n: usize) -> usize {
    n * 1024 * 1024 * 1024 * 1024 * 1024
}
pub const fn exbibytes(n: usize) -> usize {
    n * 1024 * 1024 * 1024 * 1024 * 1024 * 1024
}
pub const fn zebibytes(n: usize) -> usize {
    n * 1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024
}
