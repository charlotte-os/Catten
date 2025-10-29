mod oom_handler;

use lazy_static::lazy_static;
use oom_handler::ExtendAndClaimOnOom;
use spin::Mutex;
use spinning_top::RawSpinlock;
use talc::{ErrOnOom, OomHandler, Span, Talc, Talck};

use super::PHYSICAL_FRAME_ALLOCATOR;
use super::vmem::{MemoryMapping, VAddr};
use crate::cpu::isa::interface::memory::address::VirtualAddress;
use crate::cpu::isa::interface::memory::{AddressSpaceInterface, MemoryInterface};
use crate::cpu::isa::memory::MemoryInterfaceImpl;
use crate::cpu::isa::memory::address::VADDR_SIG_BITS;
use crate::cpu::isa::memory::paging::{AddressSpace, PAGE_SIZE};

lazy_static! {
    pub static ref ALLOCATOR_SPAN: Mutex<Span> = Mutex::new(Span::empty());
    // The first address of the higher half has the MSB and all parity bits above it set to 1.
    // We get this by using the LSH-decrement technique to set all bits before the MSB and then inverting the result.
    static ref HIGHER_HALF_START: VAddr = VAddr::from(!((1 << (*VADDR_SIG_BITS - 1)) - 1)); // 64-bit higher half start address
    static ref HIGHER_HALF_END: VAddr = VAddr::from(0xffff_ffff_ffff_ffff); // 64-bit higher half end address
}

#[global_allocator]
pub static mut KERNEL_ALLOCATOR: Talck<RawSpinlock, ExtendAndClaimOnOom> =
    Talc::new(ExtendAndClaimOnOom).lock::<RawSpinlock>();

const KERNEL_HEAP_PAGE_COUNT: usize = 8192; // 32 MiB kernel heap size

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
