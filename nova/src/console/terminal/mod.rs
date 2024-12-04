//! Kernel Terminals
//! 
//! A kernel terminal is a bytewise data source and sink that is used by the kernel to perform primitive I/O.
//! An instance can be backed by any data source and sink that implement the `Read` and `Write` traits respectively.
//! There are two variants of the terminal: 
//! - Unified I/O Terminals:
//!     - These terminals use the same data source and sink for both reading and writing.
//!     - This variant is created by directly implementing the `Terminal` trait for I/O device descriptors.
//! - Split I/O Terminals:
//!     - These terminals use different data sources and sinks for reading and writing.
//!     - This variant is created by substituting input and output device descriptor types for the respective generic
//!       parameters of the `SplitIoTerminal` struct.

use core::fmt::Write;
use crate::common::io::Read;

pub trait Terminal: Read + Write {}
pub struct SplitIoTerminal<'a, I, O>
where
I: Read,
O: Write {
    input: &'a mut I,
    output: &'a mut O,
}

impl<'a, I, O> SplitIoTerminal<'a, I, O>
where
I: Read,
O: Write {
    pub fn new(input: &'a mut I, output: &'a mut O) -> Self {
        SplitIoTerminal::<'a, I, O> {
            input,
            output
        }
    }
}

impl<'a, I, O> Write for SplitIoTerminal<'a, I, O>
where
I: Read,
O: Write {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        self.output.write_str(s)
    }
}

impl<'a, I, O> Read for SplitIoTerminal<'a, I, O>
where
I: Read,
O: Write {
    fn read(&mut self, buf: &mut [u8]) -> usize {
        self.input.read(buf);
        buf.len()
    }
}