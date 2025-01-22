pub mod address;

use crate::llk::isa::interface::memory::MemoryInterface;

pub struct MemoryInterfaceImpl;

impl MemoryInterface for MemoryInterfaceImpl {
    type AddressSpace = AddressSpace;
    type Error = Error;
    type PAddr = address::paddr::PAddr;
    type VAddr = address::vaddr::VAddr;
    type Error = Error;

/*  fn find_free_region(addr_space: usize, n_pages: usize) -> Result<Self::VAddr, Self::Error>;
    fn map_page(addr_space: usize, mapping: MemoryMapping) -> Result<(), Self::Error>;
    fn unmap_page(addr_space: usize, vaddr: Self::VAddr) -> Result<MemoryMapping, Self::Error>;
    fn is_mapped(addr_space: usize, vaddr: Self::VAddr) -> Result<bool, Self::Error>;
    fn get_mapping(addr_space: usize, vaddr: Self::VAddr) -> Result<MemoryMapping, Self::Error>;
*/
}

pub enum Error {

}

pub enum Error {}

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
