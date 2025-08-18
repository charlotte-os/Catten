//! # Page Table Hierarchy Walker
//!
//! This module implements the page table hierarchy walker for the x86_64 architecture.
//! This structure performs the actual page table walk, translating virtual addresses to physical
//! addresses, mapping pages, and unmapping pages as well as adding and removing page table entries
//! and page tables as needed.

use core::ptr::addr_of_mut;

use super::{PAGE_SIZE, is_pagetable_unused};
use crate::isa::interface::memory::address::VirtualAddress;
use crate::isa::interface::memory::{AddressSpaceInterface, MemoryInterface};
use crate::isa::x86_64::memory::address::paddr::PAddr;
use crate::isa::x86_64::memory::address::vaddr::VAddr;
use crate::memory::pmem::PHYSICAL_FRAME_ALLOCATOR;
use crate::memory::vmem::{MemoryMapping, PageType};

const CR3_ADDRESS_MASK: u64 = 0x000ffffffffff000;

pub struct PthWalker<'vas> {
    pub address_space: &'vas mut super::AddressSpace,
    pub vaddr: VAddr,
    pub pml4_ptr: *mut super::PageTable,
    pub pdpt_ptr: *mut super::PageTable,
    pub pd_ptr: *mut super::PageTable,
    pub pt_ptr: *mut super::PageTable,
    pub page_frame_ptr: *mut [u8; super::PAGE_SIZE],
}

impl<'vas> PthWalker<'vas> {
    pub fn new(address_space: &'vas mut super::AddressSpace, vaddr: VAddr) -> Self {
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

    pub fn walk(
        &mut self,
    ) -> Result<(), <super::MemoryInterfaceImpl as super::MemoryInterface>::Error> {
        self.pml4_ptr = PAddr::try_from((self.address_space.cr3 & CR3_ADDRESS_MASK) as usize)
            .unwrap()
            .into();
        self.pdpt_ptr = unsafe {
            let pml4e = &mut (*self.pml4_ptr)[self.vaddr.pml4_index()];
            if !pml4e.is_present() {
                return Err(<super::MemoryInterfaceImpl as MemoryInterface>::Error::Unmapped);
            }
            pml4e.try_get_frame().unwrap().into()
        };
        self.pd_ptr = unsafe {
            let pdpte = &mut (*self.pdpt_ptr)[self.vaddr.pdpt_index()];
            if !pdpte.is_present() {
                return Err(<super::MemoryInterfaceImpl as MemoryInterface>::Error::Unmapped);
            }
            pdpte.try_get_frame().unwrap().into()
        };
        self.pt_ptr = unsafe {
            let pde = &mut (*self.pd_ptr)[self.vaddr.pd_index()];
            if !pde.is_present() {
                return Err(<super::MemoryInterfaceImpl as MemoryInterface>::Error::Unmapped);
            }
            pde.try_get_frame().unwrap().into()
        };
        self.page_frame_ptr = unsafe {
            let pte = &mut (*self.pt_ptr)[self.vaddr.pt_index()];
            if !pte.is_present() {
                return Err(<super::MemoryInterfaceImpl as MemoryInterface>::Error::Unmapped);
            }
            pte.try_get_frame().unwrap().into()
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
                    // Obtain the PML4 table pointer; all address spaces must have a top level page
                    // table as they are all required to map the kernel and
                    // higher half memory.
                    if self.address_space.cr3 & CR3_ADDRESS_MASK == 0 {
                        let new_pml4 = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame().unwrap();
                        self.address_space.cr3 =
                            <PAddr as Into<u64>>::into(new_pml4) & CR3_ADDRESS_MASK;
                        self.address_space
                            .load()
                            .expect("Error reloading the CR3 register");
                    }
                    self.pml4_ptr =
                        PAddr::try_from((self.address_space.cr3 & CR3_ADDRESS_MASK) as usize)
                            .unwrap()
                            .into();
                    unsafe {
                        core::ptr::write_bytes(self.pml4_ptr, 0, PAGE_SIZE);
                    }
                }
                if self.pdpt_ptr.is_null() {
                    // Allocate a new page table for the PDPT
                    let new_pdpt = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame().unwrap();
                    unsafe {
                        (*self.pml4_ptr)[self.vaddr.pml4_index()]
                            .set_frame(new_pdpt)
                            .set_present(true)
                            .set_writable(writable)
                            .set_user_accessible(user_accessible)
                            .set_execute_disabled(no_execute);
                    }
                    self.pdpt_ptr = new_pdpt.into();
                    unsafe {
                        core::ptr::write_bytes(self.pdpt_ptr, 0, PAGE_SIZE);
                    }
                }
                if self.pd_ptr.is_null() {
                    // Allocate a new page table for the PD
                    let new_pd = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame().unwrap();
                    unsafe {
                        (*self.pdpt_ptr)[self.vaddr.pdpt_index()]
                            .set_frame(new_pd)
                            .set_present(true)
                            .set_writable(writable)
                            .set_user_accessible(user_accessible)
                            .set_execute_disabled(no_execute);
                    }
                    self.pd_ptr = new_pd.into();
                    unsafe {
                        core::ptr::write_bytes(self.pd_ptr, 0, PAGE_SIZE);
                    }
                }
                if self.pt_ptr.is_null() {
                    // Allocate a new page table for the PT
                    let new_pt = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame().unwrap();
                    unsafe {
                        (*self.pd_ptr)[self.vaddr.pd_index()]
                            .set_frame(new_pt)
                            .set_present(true)
                            .set_writable(writable)
                            .set_user_accessible(user_accessible)
                            .set_execute_disabled(no_execute);
                    }
                    self.pt_ptr = new_pt.into();
                    unsafe {
                        core::ptr::write_bytes(self.pt_ptr, 0, PAGE_SIZE);
                    }
                }
                // Map the page frame
                unsafe {
                    (*self.pt_ptr)[self.vaddr.pt_index()]
                        .set_frame(frame)
                        .set_present(true)
                        .set_writable(writable)
                        .set_user_accessible(user_accessible)
                        .set_execute_disabled(no_execute);
                    core::ptr::write_bytes(<PAddr as Into<*mut u8>>::into(frame), 0, PAGE_SIZE);
                }
                self.address_space
                    .load()
                    .expect("Failed to reload the address space");
                unsafe {
                    core::arch::asm!("invlpg [{}]", in(reg) self.vaddr.into_ptr::<u8>());
                }

                Ok(())
            }
            Err(other) => Err(other),
        }
    }

    pub fn unmap_page(
        &mut self,
    ) -> Result<PAddr, <super::MemoryInterfaceImpl as MemoryInterface>::Error> {
        match self.walk() {
            Ok(_) => {
                unsafe {
                    // get the return value
                    let paddr = (*self.pt_ptr)[self.vaddr.pt_index()]
                        .try_get_frame()
                        .unwrap();
                    // deallocate all higher level tables that are now unused
                    let pte = &raw mut (*self.pt_ptr)[self.vaddr.pt_index()];
                    if (*pte).is_present() {
                        // We do not deallocate the page frame here, as it is the responsibility of
                        // the VMM client calling this function to deallocate the frame if they need
                        // to.
                        (*pte).set_present(false);
                    }

                    let pde = &raw mut (*self.pd_ptr)[self.vaddr.pd_index()];
                    if is_pagetable_unused(self.pt_ptr) {
                        PHYSICAL_FRAME_ALLOCATOR
                            .lock()
                            .deallocate_frame((*pde).try_get_frame().unwrap())
                            .unwrap();
                        (*pde).set_present(false);
                    }

                    let pdpte = &raw mut (*self.pdpt_ptr)[self.vaddr.pdpt_index()];
                    if is_pagetable_unused(self.pd_ptr) {
                        PHYSICAL_FRAME_ALLOCATOR
                            .lock()
                            .deallocate_frame((*pdpte).try_get_frame().unwrap())
                            .unwrap();
                        (*pdpte).set_present(false);
                    }

                    let pml4e = &raw mut (*self.pml4_ptr)[self.vaddr.pml4_index()];
                    if is_pagetable_unused(self.pdpt_ptr) {
                        PHYSICAL_FRAME_ALLOCATOR
                            .lock()
                            .deallocate_frame((*pml4e).try_get_frame().unwrap())
                            .unwrap();
                        (*pml4e).set_present(false);
                    }
                    return Ok(paddr);
                }
            }
            Err(other) => Err(other),
        }
    }
}
