pub mod address;

pub struct MemoryInterfaceImpl;

impl interface::Interface for MemoryInterfaceImpl {
    type VAddr = address::vaddr::VAddr;
    type PAddr = address::paddr::PAddr;
}