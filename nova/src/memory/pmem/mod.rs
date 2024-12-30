use lazy_static::lazy_static;
use limine::response::MemoryMapResponse;

use crate::hal::environment::boot_protocol::limine::HHDM_REQUEST;
use crate::hal::isa::current_isa::memory::MemoryInterfaceImpl;
use crate::hal::isa::interface::memory::address::PhysicalAddress;
#[cfg(target_arch = "x86_64")]
use crate::hal::isa::interface::memory::MemoryInterface;
use crate::hal::isa::x86_64::memory::address::paddr::PAddr;

pub type VAddr = <MemoryInterfaceImpl as MemoryInterface>::VAddr;

lazy_static! {
    pub static ref HHDM_BASE: VAddr = if let Some(response) = HHDM_REQUEST.get_response() {
        return VAddr::from(response.offset() as usize);
    } else {
        panic!("Limine failed to provide a higher half direct mapping region.");
    };
}
#[derive(Debug)]
pub enum Error {
    UnableToAllocateTrackingStructure,
}

pub struct PhysicalFrameAllocator {
    huge_page_map: &'static [u8],
    large_page_map: &'static [u8],
    page_map: &'static [u8],
}

impl PhysicalFrameAllocator {}

// There should be a From implementation for each type of memory map we support.

impl From<&limine::response::MemoryMapResponse> for PhysicalFrameAllocator {
    fn from(response: &MemoryMapResponse) -> Self {
        let bitmap_size = compute_bitmap_size(response);
        let large_bitmap_size = bitmap_size / 512 + 1;
        let huge_bitmap_size = large_bitmap_size / 512 + 1;

        let bitmap_addr: PAddr = find_mmap_best_fit(response, bitmap_size).unwrap();
        let large_bitmap_addr: PAddr = find_mmap_best_fit(response, large_bitmap_size).unwrap();
        let huge_bitmap_addr: PAddr = find_mmap_best_fit(response, huge_bitmap_size).unwrap();

        todo!()
    }
}

fn compute_bitmap_size(mmap: &MemoryMapResponse) -> usize {
    let mut highest_address: PAddr = PAddr::from(0);
    // Find the highest address in the memory map.
    for entry in mmap.entries().iter() {
        let entry_end = entry.base + entry.length;
        if entry_end > <PAddr as Into<usize>>::into(highest_address) as u64 {
            highest_address = PAddr::from(entry_end as usize);
        }
    }

    (<PAddr as Into<usize>>::into(highest_address) / 4096 + 1) / 8 + 1
}

fn find_mmap_best_fit(mmap: &MemoryMapResponse, size: usize) -> Result<PAddr, Error> {
    let mut best_fit = PAddr::from(0);
    let mut best_fit_size = 0;
    for entry in mmap.entries().iter() {
        let entry_size = entry.length;
        if entry_size >= size as u64 && (best_fit_size == 0 || entry_size < best_fit_size) {
            best_fit = PAddr::from(entry.base as usize);
            best_fit_size = entry_size;
        }
    }
    if best_fit == PAddr::from(0) {
        Err(Error::UnableToAllocateTrackingStructure)
    } else {
        Ok(best_fit)
    }
}