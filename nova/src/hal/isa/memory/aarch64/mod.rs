pub mod address;

pub struct MemoryInterfaceImpl;

impl interface::Interface for MemoryInterfaceImpl {
    type PAddr = address::paddr::PAddr;
    type VAddr = address::vaddr::VAddr;
}
