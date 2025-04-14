//! Raw Data Structures
//!
//! This module provides raw data structures that handle memory allocation and deallocation through
//! the use of function pointers provided upon construction. They exist solely for use in
//! implementing the kernel allocator. The rest of the kernel should use the more thoroughly
//! developed data structures in the ds module which make use of instances of the kernel allocator.

/// For singular values
pub mod raw_box;
/// For multiple values
pub mod raw_vec;
