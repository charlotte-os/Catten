use crate::cpu::isa::interface::memory::AddressSpaceInterface;
use crate::cpu::isa::memory::paging::PAGE_SIZE;
use crate::logln;
use crate::memory::pmem::*;
use crate::memory::vmem::{MemoryMapping, PageType, VAddr};
use crate::memory::{ADDRESS_SPACE_TABLE, KERNEL_ASID, PHYSICAL_FRAME_ALLOCATOR, pmem};

pub enum Error {
    PfaError(pmem::Error),
    IsaMemoryError(crate::cpu::isa::memory::Error),
}

impl From<pmem::Error> for Error {
    fn from(err: pmem::Error) -> Self {
        Error::PfaError(err)
    }
}

impl From<crate::cpu::isa::memory::Error> for Error {
    fn from(err: crate::cpu::isa::memory::Error) -> Self {
        Error::IsaMemoryError(err)
    }
}

fn try_allocate_and_map_range(base: VAddr, num_pages: usize) -> Result<(), Error> {
    // lock the kernel address space for writing
    let kas_lock_ptr = ADDRESS_SPACE_TABLE.try_get_element_arc(KERNEL_ASID).unwrap();
    let mut kas = kas_lock_ptr.write();
    let mut mapping = MemoryMapping {
        vaddr: VAddr::default(),
        paddr: PAddr::default(),
        page_type: PageType::KernelData,
    };
    // allocate and map the pages
    // if mapping fails, deallocate and unmap the frames that were allocated
    for page_idx in 0..num_pages {
        let frame = match PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame() {
            Ok(f) => f,
            Err(err) => {
                // release the lock so the unmap_and_deallocate_range function can acquire it
                drop(kas);
                unmap_and_deallocate_range(base, page_idx);
                return Err(Error::PfaError(err));
            }
        };
        let vaddr = base + (page_idx * PAGE_SIZE) as isize;
        mapping.vaddr = vaddr;
        mapping.paddr = frame;
        if let Err(err) = kas.map_page(mapping.clone()) {
            // release the lock so the unmap_and_deallocate_range function can acquire it
            drop(kas);
            // deallocate and unmap the frames that were allocated
            unmap_and_deallocate_range(base, page_idx + 1);
            // deallocate the frame that was just allocated
            if let Err(err) = PHYSICAL_FRAME_ALLOCATOR.lock().deallocate_frame(frame) {
                logln!("Error deallocating frame at {frame:?} during cleanup: {err:?}");
            }
            return Err(Error::IsaMemoryError(err));
        }
    }
    Ok(())
}

fn unmap_and_deallocate_range(base: VAddr, num_pages: usize) {
    let kas_lock_ptr = ADDRESS_SPACE_TABLE.try_get_element_arc(KERNEL_ASID).unwrap();
    let mut kas = kas_lock_ptr.write();
    for page_idx in 0..num_pages {
        let vaddr = base + (page_idx * PAGE_SIZE) as isize;
        if let Ok(paddr) = kas.translate_address(vaddr) {
            if let Err(err) = PHYSICAL_FRAME_ALLOCATOR.lock().deallocate_frame(paddr) {
                logln!("Error deallocating frame at {paddr:?} during cleanup: {err:?}");
            }
            if let Err(err) = kas.unmap_page(vaddr) {
                logln!("Error unmapping vaddr {vaddr:?} during cleanup: {err:?}");
            }
        }
    }
}
