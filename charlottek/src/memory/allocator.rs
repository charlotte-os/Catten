use lazy_static::lazy_static;
use spin::Mutex;
use talc::{ErrOnOom, Span, Talc, Talck};

use super::pmem::PHYSICAL_FRAME_ALLOCATOR;
use super::vmem::{MemoryMapping, VAddr};
use crate::isa::current_isa::memory::MemoryInterfaceImpl;
use crate::isa::current_isa::memory::paging::AddressSpace;
use crate::isa::interface::memory::address::VirtualAddress;
use crate::isa::interface::memory::{AddressSpaceInterface, MemoryInterface};
use crate::isa::x86_64::memory::address::VADDR_SIG_BITS;
use crate::isa::x86_64::memory::paging::PAGE_SIZE;
use crate::klib::raw_mutex::RawMutex;

lazy_static! {
    pub static ref ALLOCATOR_SPAN: Mutex<Span> = Mutex::new(Span::empty());
    pub static ref KERNEL_ALLOCATOR: Talck<RawMutex, ErrOnOom> =
        Talc::new(ErrOnOom).lock::<RawMutex>();
    static ref HIGHER_HALF_START: VAddr = VAddr::from(0xffff_ffff_8000_0000); // 64-bit higher half start address
    static ref HIGHER_HALF_END: VAddr = VAddr::from(0xffff_ffff_ffff_ffff); // 64-bit higher half end address
}

const KERNEL_HEAP_N_PAGES: usize = 10; // 4 MiB kernel heap size

pub fn init_allocator() -> Result<(), ()> {
    let kernel_heap_start = <MemoryInterfaceImpl as MemoryInterface>::AddressSpace::get_current()
        .find_free_region(KERNEL_HEAP_N_PAGES, (*HIGHER_HALF_START, *HIGHER_HALF_END))
        .expect("Failed to find free region for kernel heap");
    let kernel_heap_size = KERNEL_HEAP_N_PAGES * PAGE_SIZE;

    let kernel_heap_span = Span::new(
        kernel_heap_start.into_mut(),
        (kernel_heap_start + kernel_heap_size as isize).into_mut(),
    );

    let mut address_space = AddressSpace::get_current();
    for i in (kernel_heap_start..(kernel_heap_start + kernel_heap_size as isize)).step_by(PAGE_SIZE)
    {
        let frame = PHYSICAL_FRAME_ALLOCATOR
            .lock()
            .allocate_frame()
            .expect("Failed to allocate frame for kernel heap");

        address_space
            .map_page(MemoryMapping {
                vaddr: i,
                paddr: frame,
                page_type: crate::memory::vmem::PageType::KernelData,
            })
            .expect("Failed to map page for kernel heap");
    }

    match unsafe { KERNEL_ALLOCATOR.lock().claim(kernel_heap_span) } {
        Ok(alloc_span) => {
            *ALLOCATOR_SPAN.lock() = alloc_span;
            Ok(())
        }
        Err(_) => Err(()),
    }
}
