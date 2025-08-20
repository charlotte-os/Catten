pub mod pte;
pub mod pth_walker;
pub mod tlb;

use alloc::collections::btree_map::BTreeMap;
use alloc::vec::Vec;
use core::arch::asm;
use core::iter::Iterator;

use spin::RwLock;

use super::super::lp_control::LpControl;
use super::MemoryInterfaceImpl;
use super::address::vaddr::VAddr;
use crate::isa::interface::lp_control::LpControlIfce;
use crate::isa::interface::memory::{AddressSpaceInterface, MemoryInterface, MemoryMapping};
use crate::logln;
use crate::memory::pmem::PAddr;

pub static ADDRESS_SPACE_TABLE: Vec<RwLock<AddressSpace>> = Vec::new();

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
    /* This is a map from Logical Processor IDs as assigned by this kernel to their corresponding Process Context Identifiers for the
    address space, if any. The lower 12 bits of the u16 for a given LP should be ANDed with
    the CR3 value before loading an address space on that LP */
    pcids: BTreeMap<<LpControl as LpControlIfce>::LpId, u16>,
    // control register 3 i.e. top level page table base register
    cr3: u64,
}

impl AddressSpaceInterface for AddressSpace {
    fn get_current() -> Self {
        let cr3: u64;
        unsafe {
            asm!("mov {}, cr3", out(reg) cr3);
        }
        AddressSpace {
            pcids: BTreeMap::new(),
            cr3: cr3,
        }
    }

    fn load(&self) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error> {
        unsafe {
            // Set the top level page table base register
            asm!("mov cr3, {}", in(reg) self.cr3);
        }
        Ok(())
    }

    fn find_free_region(
        &mut self,
        n_pages: usize,
        range: (VAddr, VAddr),
    ) -> Result<
        <MemoryInterfaceImpl as MemoryInterface>::VAddr,
        <MemoryInterfaceImpl as MemoryInterface>::Error,
    > {
        logln!(
            "Finding free region of {} pages in range {:?}...",
            n_pages,
            range
        );
        let mut page_iter = (range.0..range.1).step_by(PAGE_SIZE);
        while let Some(base) = page_iter.next() {
            logln!("Checking base address: {:?}", base);
            for nth_page in 0..n_pages {
                let curr_page = base + ((nth_page * PAGE_SIZE) as isize);
                //logln!("Checking page: {:?}", curr_page);
                if range.1 - curr_page < (n_pages * PAGE_SIZE) as isize {
                    return Err(<MemoryInterfaceImpl as MemoryInterface>::Error::NoRequestedVAddrRegionAvailable);
                }
                if self.is_mapped(curr_page)? {
                    match page_iter.advance_by(nth_page) {
                        Ok(_) => {
                            logln!("Page {:?} is already mapped, skipping to next base address.", curr_page);
                            break;
                        }
                        Err(_) => return Err(<MemoryInterfaceImpl as MemoryInterface>::Error::NoRequestedVAddrRegionAvailable),
                    }
                }
                if nth_page == n_pages - 1 {
                    logln!("Found free region starting at: {:?}", base);
                    return Ok(base);
                }
            }
        }
        Err(<MemoryInterfaceImpl as MemoryInterface>::Error::NoRequestedVAddrRegionAvailable)
    }

    fn map_page(
        &mut self,
        mapping: MemoryMapping,
    ) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error> {
        let mut walker = pth_walker::PthWalker::new(self, mapping.vaddr);
        walker.map_page(
            mapping.paddr,
            mapping.page_type.is_writable(),
            mapping.page_type.is_user_accessible(),
            mapping.page_type.is_no_execute(),
        )?;
        Ok(())
    }

    fn unmap_page(
        &mut self,
        vaddr: <MemoryInterfaceImpl as MemoryInterface>::VAddr,
    ) -> Result<PAddr, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        if <VAddr as Into<usize>>::into(vaddr) == 0 {
            return Err(<MemoryInterfaceImpl as MemoryInterface>::Error::NullVAddrNotAllowed);
        }
        if vaddr.page_offset() != 0 {
            return Err(<MemoryInterfaceImpl as MemoryInterface>::Error::VAddrNotPageAligned);
        }
        let mut walker = pth_walker::PthWalker::new(self, vaddr);
        walker.unmap_page()
    }

    fn is_mapped(
        &mut self,
        vaddr: <MemoryInterfaceImpl as MemoryInterface>::VAddr,
    ) -> Result<bool, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        let mut walker = pth_walker::PthWalker::new(self, vaddr);
        match walker.walk() {
            Ok(_) => Ok(true),
            Err(<MemoryInterfaceImpl as MemoryInterface>::Error::Unmapped) => Ok(false),
            Err(e) => Err(e),
        }
    }

    fn translate_address(
        &mut self,
        vaddr: super::address::vaddr::VAddr,
    ) -> Result<super::address::paddr::PAddr, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        let mut walker = pth_walker::PthWalker::new(self, vaddr);
        walker.walk()?;
        let paddr = unsafe { (*(walker.pt_ptr))[vaddr.pt_index()].try_get_frame()?.into() };
        Ok(paddr)
    }
}
