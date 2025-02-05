use core::arch::asm;

use crate::llk::isa::interface::memory::{AddressSpaceInterface, MemoryInterface, MemoryMapping};

pub struct AddressSpace {
    // control register 3 i.e. page table base register
    cr3: u64,
}

impl AddressSpaceInterface for AddressSpace {
    fn get_current() -> Self {
        let cr3: u64;
        unsafe {
            asm!("mov {}, cr3", out(reg) cr3);
        }
        AddressSpace { cr3 }
    }

    fn load(&self) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error> {
        unsafe {
            asm!("mov cr3, {}", in(reg) self.cr3);
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