//! # Page Table Hierarchy Walker
//!
//! This module implements the page table hierarchy walker for the x86_64 architecture.
//! This structure performs the actual page table walk, translating virtual addresses to physical addresses,
//! mapping pages, and unmapping pages as well as adding and removing page table entries and page tables as needed.

use core::ptr::addr_of_mut;

use super::is_pagetable_unused;
use crate::llk::isa::interface::memory::address::{PhysicalAddress, VirtualAddress};
use crate::llk::isa::interface::memory::MemoryInterface;
use crate::llk::isa::x86_64::memory::address::paddr::PAddr;
use crate::llk::isa::x86_64::memory::address::vaddr::VAddr;
use crate::memory::pmem::PHYSICAL_FRAME_ALLOCATOR;

const CR3_ADDRESS_MASK: u64 = 0x000ffffffffff000;

pub struct PthWalker<'vas> {
    address_space: &'vas super::AddressSpace,
    vaddr: VAddr,
    pml4_ptr: *mut super::PageTable,
    pdpt_ptr: *mut super::PageTable,
    pd_ptr: *mut super::PageTable,
    pt_ptr: *mut super::PageTable,
    page_frame_ptr: *mut [u8; super::PAGE_SIZE],
}

impl<'vas> PthWalker<'vas> {
    pub fn new(address_space: &'vas super::AddressSpace, vaddr: VAddr) -> Self {
        Self {
            address_space,
            vaddr,
            pml4_ptr: core::ptr::null_mut(),
            pdpt_ptr: core::ptr::null_mut(),
            pd_ptr: core::ptr::null_mut(),
            pt_ptr: core::ptr::null_mut(),
            page_frame_ptr: core::ptr::null_mut(),
        }
    }

    pub fn walk(&mut self) -> Result<(), <super::MemoryInterfaceImpl as super::MemoryInterface>::Error> {
        self.pml4_ptr = PAddr::from((self.address_space.cr3 & CR3_ADDRESS_MASK) as usize).into();
        self.pdpt_ptr = unsafe {
            let pml4e = &mut (*self.pml4_ptr)[self.vaddr.pml4_index()];
            if !pml4e.is_present() {
                return Err(<super::MemoryInterfaceImpl as MemoryInterface>::Error::Unmapped);
            }
            pml4e.get_frame().into()
        };
        self.pd_ptr = unsafe {
            let pdpte = &mut (*self.pdpt_ptr)[self.vaddr.pdpt_index()];
            if !pdpte.is_present() {
                return Err(<super::MemoryInterfaceImpl as MemoryInterface>::Error::Unmapped);
            }
            pdpte.get_frame().into()
        };
        self.pt_ptr = unsafe {
            let pde = &mut (*self.pd_ptr)[self.vaddr.pd_index()];
            if !pde.is_present() {
                return Err(<super::MemoryInterfaceImpl as MemoryInterface>::Error::Unmapped);
            }
            pde.get_frame().into()
        };
        self.page_frame_ptr = unsafe {
            let pte = &mut (*self.pt_ptr)[self.vaddr.pt_index()];
            if !pte.is_present() {
                return Err(<super::MemoryInterfaceImpl as MemoryInterface>::Error::Unmapped);
            }
            pte.get_frame().into()
        };

        Ok(())
    }

    pub fn map_page(
        &mut self,
        frame: PAddr,
        writable: bool,
        user_accessible: bool,
        no_execute: bool,
    ) -> Result<(), <super::MemoryInterfaceImpl as MemoryInterface>::Error> {
        match self.walk() {
            Ok(_) => Err(<super::MemoryInterfaceImpl as MemoryInterface>::Error::AlreadyMapped),
            Err(<super::MemoryInterfaceImpl as MemoryInterface>::Error::Unmapped) => {
                if self.pml4_ptr.is_null() {
                    // Obtain the PML4 table pointer; all address spaces must have a top level page table
                    // as they are all required to map the kernel and higher half memory.
                    if self.address_space.cr3 & CR3_ADDRESS_MASK == 0 {
                        return Err(<super::MemoryInterfaceImpl as MemoryInterface>::Error::UnmappedTopLevelPageTable);
                    }
                    self.pml4_ptr = PAddr::from((self.address_space.cr3 & CR3_ADDRESS_MASK) as usize).into();
                }
                if self.pdpt_ptr.is_null() {
                    // Allocate a new page table for the PDPT
                    let new_pdpt = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame()?;
                    unsafe {
                        (*self.pml4_ptr)[self.vaddr.pml4_index()]
                            .set_frame(new_pdpt)
                            .set_present(true)
                            .set_writable(writable)
                            .set_user_accessible(user_accessible)
                            .set_execute_disabled(no_execute);
                    }
                    self.pdpt_ptr = new_pdpt.into();
                }
                if self.pd_ptr.is_null() {
                    // Allocate a new page table for the PD
                    let new_pd = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame()?;
                    unsafe {
                        (*self.pdpt_ptr)[self.vaddr.pdpt_index()]
                            .set_frame(new_pd)
                            .set_present(true)
                            .set_writable(writable)
                            .set_user_accessible(user_accessible)
                            .set_execute_disabled(no_execute);
                    }
                    self.pd_ptr = new_pd.into();
                }
                if self.pt_ptr.is_null() {
                    // Allocate a new page table for the PT
                    let new_pt = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame()?;
                    unsafe {
                        (*self.pd_ptr)[self.vaddr.pd_index()]
                            .set_frame(new_pt)
                            .set_present(true)
                            .set_writable(writable)
                            .set_user_accessible(user_accessible)
                            .set_execute_disabled(no_execute);
                    }
                    self.pt_ptr = new_pt.into();
                }
                // Map the page frame
                let frame = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame()?;
                unsafe {
                    (*self.pt_ptr)[self.vaddr.pt_index()]
                        .set_frame(frame)
                        .set_present(true)
                        .set_writable(writable)
                        .set_user_accessible(user_accessible)
                        .set_execute_disabled(no_execute);
                }
                Ok(())
            }
            Err(other) => Err(other),
        }
    }

    pub fn unmap_page(&mut self) -> Result<(), <super::MemoryInterfaceImpl as MemoryInterface>::Error> {
        match self.walk() {
            Ok(_) => {
                unsafe {
                    let pte = addr_of_mut!((*self.pt_ptr)[self.vaddr.pt_index()]);
                    if (*pte).is_present() {
                        PHYSICAL_FRAME_ALLOCATOR.lock().deallocate_frame((*pte).get_frame())?;
                        (*pte).set_present(false);
                    }

                    let pde = addr_of_mut!((*self.pd_ptr)[self.vaddr.pd_index()]);
                    if is_pagetable_unused(self.pt_ptr) {
                        PHYSICAL_FRAME_ALLOCATOR.lock().deallocate_frame((*pde).get_frame())?;
                        (*pde).set_present(false);
                    }

                    let pdpte = addr_of_mut!((*self.pdpt_ptr)[self.vaddr.pdpt_index()]);
                    if is_pagetable_unused(self.pd_ptr) {
                        PHYSICAL_FRAME_ALLOCATOR.lock().deallocate_frame((*pdpte).get_frame())?;
                        (*pdpte).set_present(false);
                    }

                    let pml4e = addr_of_mut!((*self.pml4_ptr)[self.vaddr.pml4_index()]);
                    if is_pagetable_unused(self.pdpt_ptr) {
                        PHYSICAL_FRAME_ALLOCATOR.lock().deallocate_frame((*pml4e).get_frame())?;
                        (*pml4e).set_present(false);
                    }
                }
                Ok(())
            }
            Err(other) => Err(other),
        }
    }
}
