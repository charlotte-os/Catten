pub mod ioaddr;
pub mod paddr;
pub mod vaddr;

use super::instructions::{inb, outb};
use crate::hal::isa::interfaces::memory;

pub struct Memory;

impl crate::hal::isa::interfaces::memory::Memory for Memory {
    type VAddr = vaddr::VAddr;
    type PAddr = PAddr;
    type IoAddr = IoAddr;
}

