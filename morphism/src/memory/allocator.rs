use core::alloc::Layout;

use lazy_static::lazy_static;
use spin::Mutex;
use talc::{ErrOnOom, OomHandler, Span, Talc, Talck};

use super::pmem::PHYSICAL_FRAME_ALLOCATOR;
use super::vmem::{MemoryMapping, VAddr};
use crate::llk::environment::boot_protocol::limine::EXECUTABLE_ADDRESS_REQUEST;
use crate::llk::isa::current_isa::memory::paging::AddressSpace;
use crate::llk::isa::interface::memory::address::VirtualAddress;
use crate::llk::isa::interface::memory::AddressSpaceInterface;
use crate::llk::isa::x86_64::memory::paging::PAGE_SIZE;

lazy_static! {
    pub static ref ALLOCATOR_SPAN: Mutex<Span> = Mutex::new(Span::empty());
    pub static ref KERNEL_ALLOCATOR: Mutex<Talc<ErrOnOom>> = Mutex::new(Talc::new(ErrOnOom));
}

const KERNEL_HEAP_START: VAddr = unsafe { VAddr::from_raw_unchecked(0xffff800000000000) };
const KERNEL_HEAP_SIZE: usize = 64 * 1024 * 1024; // 64MiB

pub fn init_allocator() -> Result<(), ()> {
    let kernel_heap_start = KERNEL_HEAP_START.into_mut();
    let kernel_heap_size = KERNEL_HEAP_SIZE;

    let kernel_heap_span = Span::new(
        kernel_heap_start,
        (KERNEL_HEAP_START + kernel_heap_size as isize).into_mut(),
    );
    for i in (VAddr::from_mut(kernel_heap_start)
        ..VAddr::from_mut(kernel_heap_start.wrapping_add(kernel_heap_size)))
        .step_by(PAGE_SIZE)
    {
        let frame = PHYSICAL_FRAME_ALLOCATOR
            .lock()
            .allocate_frame()
            .expect("Failed to allocate frame for kernel heap");
        let mut address_space = AddressSpace::get_current();
        address_space
            .map_page(MemoryMapping {
                vaddr: i,
                paddr: frame.into(),
                page_type: crate::memory::vmem::PageType::KernelData,
            })
            .expect("Failed to map page for kernel heap");
    }

    let mut kernel_allocator = KERNEL_ALLOCATOR.lock();
    if let Ok(alloc_span) = unsafe { kernel_allocator.claim(kernel_heap_span) } {
        *ALLOCATOR_SPAN.lock() = alloc_span;
        Ok(())
    } else {
        Err(())
    }
}
