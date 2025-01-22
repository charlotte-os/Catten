pub mod address;

use crate::hal::isa::interface::memory::MemoryInterface;

pub struct MemoryInterfaceImpl;

impl MemoryInterface for MemoryInterfaceImpl {
    type PAddr = address::paddr::PAddr;
    type VAddr = address::vaddr::VAddr;
    type Error = Error;
}

pub enum Error {

}