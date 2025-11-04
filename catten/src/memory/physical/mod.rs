//! # Physical Memory Management
//!
//! This module is responsible for managing physical memory. It provides an interface for allocating
//! and freeing physical memory frames.

pub use limine::response::MemoryMapResponse;

use crate::common::constants::BITS_PER_BYTE;
use crate::common::size::kibibytes;
pub use crate::cpu::isa::interface::memory::MemoryInterface;
use crate::cpu::isa::interface::memory::address::Address;
pub use crate::cpu::isa::interface::memory::address::PhysicalAddress;
pub use crate::cpu::isa::memory::MemoryInterfaceImpl;
pub use crate::cpu::isa::memory::address::paddr::{PAddr, PAddrError};
use crate::logln;

/// Page frames are 4 KiB in size on all supported architectures.
const PAGE_FRAME_SIZE: usize = kibibytes(4);

#[derive(Debug, Clone, Copy)]
pub enum Error {
    UnableToAllocateTrackingStructure,
    MisalignedPhysicalAddress,
    RequestLargerThanTotalMemory,
    OutOfFrames,
    InvalidPAddr,
    InvalidPhysAlignment,
    CannotDeallocateUnallocatedFrame,
    FrameAlreadyInUse,
    NoOp,
    PAddrError(PAddrError),
}

impl From<PAddrError> for Error {
    fn from(err: PAddrError) -> Self {
        Error::PAddrError(err)
    }
}

#[derive(Debug)]
pub struct PhysicalFrameAllocator {
    bitmap_ptr: *mut u8,
    bitmap_len: usize,
}

unsafe impl Send for PhysicalFrameAllocator {}

impl PhysicalFrameAllocator {
    pub fn mark_frame_unavailable(&mut self, frame_addr: PAddr) -> Result<(), Error> {
        if <PAddr as Into<usize>>::into(frame_addr) % PAGE_FRAME_SIZE != 0 {
            return Err(Error::MisalignedPhysicalAddress);
        }
        let idx = addr_to_bitmap_index(frame_addr)?;
        let byte_idx = idx.0;
        let bit_idx = idx.1;
        unsafe {
            if self.bitmap_ptr.offset(byte_idx as isize).read_volatile() & (1 << bit_idx) != 0 {
                Err(Error::FrameAlreadyInUse)
            } else {
                // set the bit corresponding to the frame being marked unavailable
                *self.bitmap_ptr.offset(byte_idx as isize) |= 1 << bit_idx;
                return Ok(());
            }
        }
    }

    pub fn allocate_frame(&mut self) -> Result<PAddr, Error> {
        let mut curr_byte_ptr: *mut u8;
        for byte_idx in 0..self.bitmap_len {
            unsafe {
                curr_byte_ptr = self.bitmap_ptr.offset(byte_idx as isize);
                if curr_byte_ptr.read() != 0xff {
                    for bit_idx in 0..7 {
                        if curr_byte_ptr.read() & (1 << bit_idx) == 0u8 {
                            //set the bit corresponding to the allocated frame
                            curr_byte_ptr
                                .write_volatile(curr_byte_ptr.read_volatile() | (1 << bit_idx));
                            let raw_addr = (byte_idx * BITS_PER_BYTE + bit_idx) * PAGE_FRAME_SIZE;
                            return Ok(PAddr::try_from(raw_addr)?);
                        }
                    }
                }
            }
        }
        Err(Error::OutOfFrames)
    }

    #[inline]
    fn is_containing_frame_available(&self, addr: PAddr) -> Result<bool, Error> {
        let (byte_idx, bit_idx) = addr_to_bitmap_index(addr.prev_aligned_to(PAGE_FRAME_SIZE))?;
        unsafe {
            let byte = self.bitmap_ptr.offset(byte_idx as isize).read_volatile();
            Ok(byte & (1 << bit_idx) == 0)
        }
    }

    #[inline]
    fn mark_frames_unavailable(&mut self, start_addr: PAddr, nframes: usize) -> Result<(), Error> {
        for i in 0..nframes {
            self.mark_frame_unavailable(start_addr + (i * PAGE_FRAME_SIZE) as isize)?;
        }
        Ok(())
    }

    pub fn allocate_contiguous(
        &mut self,
        nframes: usize,
        alignment: usize,
    ) -> Result<PAddr, Error> {
        if nframes == 0 {
            Err(Error::NoOp)
        } else if nframes / BITS_PER_BYTE > self.bitmap_len {
            Err(Error::RequestLargerThanTotalMemory)
        } else if alignment % PAGE_FRAME_SIZE != 0
            || alignment / (PAGE_FRAME_SIZE * BITS_PER_BYTE) > self.bitmap_len
        {
            Err(Error::InvalidPhysAlignment)
        } else {
            let mut start_frame_base = alignment;

            'outer: loop {
                for fb in (start_frame_base..(start_frame_base + nframes * PAGE_FRAME_SIZE))
                    .step_by(PAGE_FRAME_SIZE)
                {
                    if !self.is_containing_frame_available(PAddr::try_from(fb as usize).unwrap())? {
                        start_frame_base += alignment;
                        break;
                    } else if fb == start_frame_base + (nframes - 1) * PAGE_FRAME_SIZE {
                        // found a suitable range
                        break 'outer;
                    }
                }
            }
            let start_addr = PAddr::try_from(start_frame_base)?;
            self.mark_frames_unavailable(start_addr, nframes)?;
            Ok(start_addr) // Placeholder
        }
    }

    pub fn deallocate_frame(&mut self, frame_addr: PAddr) -> Result<(), Error> {
        if <PAddr as Into<usize>>::into(frame_addr.clone()) % PAGE_FRAME_SIZE != 0 {
            return Err(Error::MisalignedPhysicalAddress);
        }
        if let Ok(idx) = addr_to_bitmap_index(frame_addr) {
            let byte_idx = idx.0;
            let bit_idx = idx.1;
            unsafe {
                if self.bitmap_ptr.offset(byte_idx as isize).read_volatile() & (1 << bit_idx) == 0 {
                    Err(Error::CannotDeallocateUnallocatedFrame)
                } else {
                    // clear the bit corresponding to the frame being deallocated
                    *self.bitmap_ptr.offset(byte_idx as isize) &= !(1 << bit_idx);
                    return Ok(());
                }
            }
        } else {
            Err(Error::InvalidPAddr)
        }
    }
}

// There should be a From implementation for each type of memory map we support.

impl From<&MemoryMapResponse> for PhysicalFrameAllocator {
    fn from(response: &MemoryMapResponse) -> Self {
        logln!("Computing PhysicalFrameAllocator bitmap size...");
        let bitmap_size = compute_bitmap_size(response);
        logln!("PhysicalFrameAllocator bitmap size: {:?} bytes", bitmap_size);
        logln!("Finding best fit memory location for the PhysicalFrameAllocator bitmap...");
        let bitmap_addr: PAddr = find_mmap_best_fit(response, bitmap_size).unwrap();
        logln!("PhysicalFrameAllocator bitmap addr (physical): {:?}", bitmap_addr);
        let pfa = PhysicalFrameAllocator {
            bitmap_ptr: unsafe { bitmap_addr.into_hhdm_mut::<u8>() },
            bitmap_len: bitmap_size,
        };
        // Initially mark all frames as unavailable.
        logln!("Clearing PhysicalFrameAllocator bitmap...");
        for i in 0..bitmap_size {
            unsafe {
                *(pfa.bitmap_ptr.offset(i as isize)) = 0xffu8;
            }
        }
        logln!("Initializing PhysicalFrameAllocator bitmap...");
        init_bitmap_from_mmap(pfa.bitmap_ptr, response);
        //address zero is not accessible
        unsafe {
            *pfa.bitmap_ptr |= 1;
        }
        // Mark the bitmap region as unusable.
        mark_pfa_bitmap_unusable(pfa.bitmap_ptr, bitmap_addr, bitmap_size);
        logln!("PhysicalFrameAllocator bitmap initialized.");

        pfa
    }
}

fn compute_bitmap_size(mmap: &MemoryMapResponse) -> usize {
    let mut highest_address: PAddr = unsafe { PAddr::from_unchecked(0usize) };
    // Find the highest address in the memory map.
    for entry in mmap.entries().iter() {
        let entry_end = entry.base + entry.length;
        if entry_end > <PAddr as Into<usize>>::into(highest_address) as u64 {
            highest_address = unsafe { PAddr::from_unchecked(entry_end as usize) };
        }
    }
    // Compute the size of the bitmap needed to track all frames up to the highest address.
    let haddr_raw = <PAddr as Into<usize>>::into(highest_address);
    let num_pages = haddr_raw / PAGE_FRAME_SIZE
        + if haddr_raw % PAGE_FRAME_SIZE > 0 {
            1
        } else {
            0
        };
    let num_bmap_bytes = num_pages / BITS_PER_BYTE
        + if num_pages % BITS_PER_BYTE > 0 {
            1
        } else {
            0
        };
    num_bmap_bytes
}

// Helper functions

fn find_mmap_best_fit(mmap: &MemoryMapResponse, size: usize) -> Result<PAddr, Error> {
    let mut best_fit = PAddr::try_from(0usize)?;
    let mut best_fit_size = 0;
    for entry in mmap.entries().iter() {
        let entry_size = entry.length;
        if entry_size >= size as u64 && (best_fit_size == 0 || entry_size < best_fit_size) {
            best_fit = PAddr::try_from(entry.base as usize)?;
            best_fit_size = entry_size;
        }
    }
    if best_fit == PAddr::try_from(0usize)? {
        Err(Error::UnableToAllocateTrackingStructure)
    } else {
        Ok(best_fit)
    }
}

fn addr_to_bitmap_index(addr: PAddr) -> Result<(usize, usize), Error> {
    if <PAddr as Into<usize>>::into(addr) % PAGE_FRAME_SIZE != 0 {
        return Err(Error::MisalignedPhysicalAddress);
    }

    let bit_index = <PAddr as Into<usize>>::into(addr) >> 12; // divide by PAGE_FRAME_SIZE

    let byte_index = bit_index / BITS_PER_BYTE;
    let bit_offset = bit_index % BITS_PER_BYTE;

    Ok((byte_index, bit_offset))
}

fn init_bitmap_from_mmap(bitmap_ptr: *mut u8, mmap: &MemoryMapResponse) {
    for entry in mmap.entries().iter() {
        if entry.entry_type == limine::memory_map::EntryType::USABLE {
            let start = entry.base;
            let end = entry.base + entry.length;
            for i in (start..end).step_by(PAGE_FRAME_SIZE) {
                //logln!("Marking frame at physical address {:?} as available...", i);
                let (byte_index, bit_offset) =
                    addr_to_bitmap_index(PAddr::try_from(i as usize).unwrap()).unwrap();
                unsafe {
                    *(bitmap_ptr.offset(byte_index as isize)) &= !(1 << bit_offset);
                }
            }
        }
    }
}

fn mark_pfa_bitmap_unusable(bitmap_ptr: *mut u8, base: PAddr, length: usize) {
    let n_pages = if length % PAGE_FRAME_SIZE > 0 {
        length / PAGE_FRAME_SIZE + 1
    } else {
        length / PAGE_FRAME_SIZE
    };

    for i in 0..n_pages {
        let pfa_index = addr_to_bitmap_index(base + (i * PAGE_FRAME_SIZE) as isize)
            .expect("Failed to convert PAddr to bitmap index.");
        unsafe {
            *(bitmap_ptr.offset(pfa_index.0 as isize)) |= 1 << pfa_index.1;
        }
    }
}
