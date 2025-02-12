pub mod address;

use core::arch::asm;

use crate::llk::isa::interface::memory::{AddressSpaceInterface, MemoryInterface, MemoryMapping};

pub struct MemoryInterfaceImpl;

impl MemoryInterface for MemoryInterfaceImpl {
    type AddressSpace = AddressSpace;
    type Error = Error;
    type PAddr = address::paddr::PAddr;
    type VAddr = address::vaddr::VAddr;
}

pub enum Error {}

pub struct AddressSpace {
    /// user space translation table base register
    ttbr0_el1: u64,
    /// kernel space translation table base register
    ttbr1_el1: u64,
}

impl AddressSpaceInterface for AddressSpace {
    fn get_current() -> Self {
        let ttbr0_el1: u64;
        let ttbr1_el1: u64;
        unsafe {
            asm!("mrs {}, ttbr0_el1", out(reg) ttbr0_el1);
            asm!("mrs {}, ttbr1_el1", out(reg) ttbr1_el1);
        }
        AddressSpace { ttbr0_el1, ttbr1_el1 }
    }

    fn load(&self) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error> {
        unsafe {
            asm!("msr ttbr0_el1, {}", in(reg) self.ttbr0_el1);
            asm!("msr ttbr1_el1, {}", in(reg) self.ttbr1_el1);
        }
        Ok(())
    }

    fn find_free_region(
        &self,
        n_pages: usize,
    ) -> Result<<MemoryInterfaceImpl as MemoryInterface>::VAddr, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        todo!()
    }

    fn map_page(&mut self, mapping: MemoryMapping) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error> {
        todo!()
    }

    fn unmap_page(
        &mut self,
        vaddr: <MemoryInterfaceImpl as MemoryInterface>::VAddr,
    ) -> Result<MemoryMapping, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        todo!()
    }

    fn is_mapped(
        &self,
        vaddr: <MemoryInterfaceImpl as MemoryInterface>::VAddr,
    ) -> Result<bool, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        todo!()
    }

    fn get_mapping(
        &self,
        vaddr: <MemoryInterfaceImpl as MemoryInterface>::VAddr,
    ) -> Result<MemoryMapping, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        todo!()
    }
}
