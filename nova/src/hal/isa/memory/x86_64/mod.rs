pub mod address;

pub struct MemoryInterfaceImpl;

impl super::interface::MemoryInterface for MemoryInterfaceImpl {
    type PAddr = address::paddr::PAddr;
    type VAddr = address::vaddr::VAddr;
}
