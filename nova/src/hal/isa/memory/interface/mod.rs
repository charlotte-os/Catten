pub mod address;

pub trait MemoryInterface {
    type VAddr: address::VirtualAddress;
    type PAddr: address::PhysicalAddress;
}