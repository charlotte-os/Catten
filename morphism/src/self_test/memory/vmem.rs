use core::ops::Add;

use crate::llk::isa::current_isa::memory::paging::AddressSpace;
use crate::llk::isa::interface::memory::AddressSpaceInterface;
use crate::memory::pmem::PHYSICAL_FRAME_ALLOCATOR;
use crate::memory::vmem::{MemoryMapping, PageType};

const HIGHER_HALF_START: usize = 0xffff_ffff_8000_0000;

fn test_vmem() {
    let frame = PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame().unwrap();
    let mut current_as = AddressSpace::get_current();
    let mapping = MemoryMapping {
        vaddr: HIGHER_HALF_START.into(),
        paddr: frame,
        page_type: PageType::KernelData,
    };
    current_as.map_page(mapping).expect("Error mapping page.");
}
