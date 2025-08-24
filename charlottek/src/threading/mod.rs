//! # Threading
//!
//! This module provides functionality for managing threads of execution within a process.
//! Processes own their threads and as a result there is no global thread table and the scheduler
//! uses process aware scheduling in order to keep the caches and TLBs hot and reduce context
//! switching overhead.
//!
//! This module is made up of three components:
//! - `thread`: Contains the structures used to describe and manage threads of execution.
//! - `scheduler`: Manages the assignment of threads to processors and scheduling their execution.
//! - `sync`: Provides synchronization primitives for coordinating access to shared resources
//!   suitable for use in both the kernel and user space.

pub mod scheduler;
pub mod sync;
pub mod thread;
