pub mod pte;
pub mod pth_walker;

use core::arch::asm;

use super::MemoryInterfaceImpl;
use crate::llk::isa::interface::memory::{AddressSpaceInterface, MemoryInterface, MemoryMapping};

pub const PAGE_SIZE: usize = 4096;
pub const N_PAGE_TABLE_ENTRIES: usize = 512;
pub type PageTable = [pte::PageTableEntry; N_PAGE_TABLE_ENTRIES];

pub fn is_pagetable_unused(table_ptr: *const PageTable) -> bool {
    unsafe {
        for i in 0..N_PAGE_TABLE_ENTRIES {
            if (*table_ptr)[i].is_present() {
                return false;
            }
        }
    }
    true
}

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
