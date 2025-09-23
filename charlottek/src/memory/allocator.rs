use lazy_static::lazy_static;
use spin::Mutex;
use talc::{ErrOnOom, Span, Talc, Talck};

use super::PHYSICAL_FRAME_ALLOCATOR;
use super::vmem::{MemoryMapping, VAddr};
use crate::isa::interface::memory::address::VirtualAddress;
use crate::isa::interface::memory::{AddressSpaceInterface, MemoryInterface};
use crate::isa::memory::MemoryInterfaceImpl;
use crate::isa::memory::address::VADDR_SIG_BITS;
use crate::isa::memory::paging::{AddressSpace, PAGE_SIZE};
use crate::klib::raw_mutex::RawMutex;

lazy_static! {
    pub static ref ALLOCATOR_SPAN: Mutex<Span> = Mutex::new(Span::empty());
    // The first address of the higher half has the MSB and all parity bits above it set to 1.
    // We get this by using the LSH-decrement technique to set all bits before the MSB and then inverting the result.
    static ref HIGHER_HALF_START: VAddr = VAddr::from(!((1 << (*VADDR_SIG_BITS - 1)) - 1)); // 64-bit higher half start address
    static ref HIGHER_HALF_END: VAddr = VAddr::from(0xffff_ffff_ffff_ffff); // 64-bit higher half end address
}

#[global_allocator]
pub static mut KERNEL_ALLOCATOR: Talck<RawMutex, ErrOnOom> = Talc::new(ErrOnOom).lock::<RawMutex>();

const KERNEL_HEAP_PAGE_COUNT: usize = 1024; // 4 MiB kernel heap size

pub fn init_allocator() -> Result<(), ()> {
    let kernel_heap_start = <MemoryInterfaceImpl as MemoryInterface>::AddressSpace::get_current()
        .find_free_region(KERNEL_HEAP_PAGE_COUNT, (*HIGHER_HALF_START, *HIGHER_HALF_END))
        .expect("Failed to find free region for kernel heap");
    let kernel_heap_size = KERNEL_HEAP_PAGE_COUNT * PAGE_SIZE;

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
