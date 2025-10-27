pub mod address;
pub mod paging;

use core::arch::asm;

use address::paddr::PAddr;
use address::vaddr::VAddr;

use crate::cpu::isa::interface::memory::address::{Address, PhysicalAddress, VirtualAddress};
use crate::cpu::isa::interface::memory::{AddressSpaceInterface, MemoryInterface, MemoryMapping};

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
        AddressSpace {
            ttbr0_el1,
            ttbr1_el1,
        }
    }

    fn load(&self) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error> {
        unsafe {
            asm!("msr ttbr0_el1, {}", in(reg) self.ttbr0_el1);
            asm!("msr ttbr1_el1, {}", in(reg) self.ttbr1_el1);
        }
        Ok(())
    }

    fn find_free_region(
        &mut self,
        n_pages: usize,
        range: (
            <MemoryInterfaceImpl as MemoryInterface>::VAddr,
            <MemoryInterfaceImpl as MemoryInterface>::VAddr,
        ),
    ) -> Result<
        <MemoryInterfaceImpl as MemoryInterface>::VAddr,
        <MemoryInterfaceImpl as MemoryInterface>::Error,
    > {
        // Use n_pages and range to implement the logic
        todo!()
    }

    fn map_page(
        &mut self,
        mapping: MemoryMapping,
    ) -> Result<(), <MemoryInterfaceImpl as MemoryInterface>::Error> {
        todo!()
    }

    fn unmap_page(
        &mut self,
        vaddr: <MemoryInterfaceImpl as MemoryInterface>::VAddr,
    ) -> Result<MemoryMapping, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        todo!()
    }

    fn is_mapped(
        &mut self,
        vaddr: <MemoryInterfaceImpl as MemoryInterface>::VAddr,
    ) -> Result<bool, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        todo!()
    }

    fn translate_address(
        &mut self,
        vaddr: VAddr,
    ) -> Result<PAddr, <MemoryInterfaceImpl as MemoryInterface>::Error> {
        let mut par_el1 = (0u64, 0u64);
        unsafe {
            asm!(
                // Address translation stage 1 at EL1 without permission check
                "at s1e1a, {}",
                "isb", // Weakly ordered ISA is weakly ordered lol
                "mrrs x0, x1, par_el1",
                vaddr = in(reg) vaddr,
                lateout("x0") par_el1.0,
                lateout("x1") par_el1.1,
            );
        }
        if par_el1.0 & 1 == 1 {
            // Check F bit
            Err(Error {})
        } else {
            Ok(PAddr(
                if is_d128_set(par_el1) {
                    par_el1.1
                } else {
                    par_el1.0
                } & PAR_EL1_PADDR_MASK,
            ))
        }
    }
}

const PAR_EL1_PADDR_MASK: u64 = 0x0000fffffffff000;

fn is_d128_set(par_el1: (u64, u64)) -> bool {
    par_el1.1 & 1 == 1
}
