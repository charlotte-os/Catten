//! # Physical Memory Management
//!
//! This module is responsible for managing physical memory. It provides an interface for allocating and freeing
//! physical memory frames.

use core::mem::MaybeUninit;

use lazy_static::lazy_static;
use limine::response::MemoryMapResponse;
use spin::Mutex;

use crate::hal::environment::boot_protocol::limine::{HHDM_REQUEST, MEMEORY_MAP_REQUEST};
use crate::hal::isa::current_isa::memory::address::paddr::PAddr;
use crate::hal::isa::current_isa::memory::MemoryInterfaceImpl;
use crate::hal::isa::interface::memory::MemoryInterface;
use crate::logln;

pub type VAddr = <MemoryInterfaceImpl as MemoryInterface>::VAddr;

lazy_static! {
    pub static ref HHDM_BASE: VAddr = if let Some(response) = HHDM_REQUEST.get_response() {
        return VAddr::from(response.offset() as usize);
    } else {
        panic!("Limine failed to provide a higher half direct mapping region.");
    };
    pub static ref PHYSICAL_FRAME_ALLOCATOR: Mutex<PhysicalFrameAllocator> = Mutex::new(PhysicalFrameAllocator::from(MEMEORY_MAP_REQUEST.get_response().unwrap()));
}

#[derive(Debug)]
pub enum Error {
    UnableToAllocateTrackingStructure,
    MisalignedPhysicalAddress,
}

#[derive(Debug)]
pub struct PhysicalFrameAllocator {
    bitmap_ptr: *mut u8,
    bitmap_len: usize,
}

unsafe impl Send for PhysicalFrameAllocator {}

impl PhysicalFrameAllocator {

}

// There should be a From implementation for each type of memory map we support.

impl From<&MemoryMapResponse> for PhysicalFrameAllocator {
    fn from(response: &MemoryMapResponse) -> Self {
        logln!("Computing PhysicalFrameAllocator bitmap size...");
        let bitmap_size = compute_bitmap_size(response);
        logln!("PhysicalFrameAllocator bitmap size: {:?}", bitmap_size);
        logln!("Finding best fit memory location for the PhysicalFrameAllocator bitmap...");
        let bitmap_addr: PAddr = find_mmap_best_fit(response, bitmap_size).unwrap();
        logln!("PhysicalFrameAllocator bitmap addr (physical): {:?}", bitmap_addr);
        let pfa = PhysicalFrameAllocator {
            bitmap_ptr: bitmap_addr.into(),
            bitmap_len: bitmap_size
        };
        // Initially mark all frames as unavailable.
        for i in 0..bitmap_size {
            unsafe {
                *pfa.bitmap_ptr.add(i) = 0xFF;
            }
        }
        logln!("Initializing PhysicalFrameAllocator bitmap...");
        init_bitmap_from_mmap(pfa.bitmap_ptr, pfa.bitmap_len, response);
        logln!("PhysicalFrameAllocator bitmap initialized.");

        pfa
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

fn addr_to_bitmap_index(addr: PAddr) -> Result<(usize, usize), Error> {
    if <PAddr as Into<usize>>::into(addr) % 4096 != 0 {
        return Err(Error::MisalignedPhysicalAddress);
    }

    let bit_index = <PAddr as Into<usize>>::into(addr) / 4096;

    let byte_index = bit_index / 8;
    let bit_offset = bit_index % 8;

    Ok((byte_index, bit_offset))
}

fn init_bitmap_from_mmap(bitmap_ptr: *mut u8, bitmap_len: usize, mmap: &MemoryMapResponse) {
    for entry in mmap.entries().iter() {
        let start = entry.base;
        let end = entry.base + entry.length;
        let start_index = start / 4096;
        let end_index = end / 4096;
        for i in (start_index..end_index).step_by(4096) {
            let (byte_index, bit_offset) = addr_to_bitmap_index(PAddr::from(i as usize)).unwrap();
            unsafe {
                *(bitmap_ptr.offset(byte_index as isize)) &= 0 << bit_offset;
            }
        }
    }
}
