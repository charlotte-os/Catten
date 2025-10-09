use alloc::alloc::Layout;
use core::cmp::max;

use talc::{OomHandler, Span, Talc};

use super::ALLOCATOR_SPAN;
use crate::cpu::isa::interface::memory::AddressSpaceInterface;
use crate::cpu::isa::memory::paging::PAGE_SIZE;
use crate::memory::{ADDRESS_SPACE_TABLE, KERNEL_ASID, VAddr};
pub struct ExtendAndClaimOnOom;

impl OomHandler for ExtendAndClaimOnOom {
    fn handle_oom(talc: &mut Talc<Self>, layout: Layout) -> Result<(), ()> {
        // extend by the larger of a page or the requested size
        let exp_size = max(PAGE_SIZE, layout.size());
        let additional_span = extend_allocator_arena(exp_size)?;
        let mut alloc_span = ALLOCATOR_SPAN.lock();
        let new_span = alloc_span.extend(0, additional_span.size());
        let claimed_span = unsafe { talc.claim(new_span)? };
        *alloc_span = claimed_span;
        Ok(())
    }
}

fn extend_allocator_arena(size: usize) -> Result<Span, ()> {
    let span_base_acme =
        ALLOCATOR_SPAN.lock().get_base_acme().expect("Allocator span not initialized");
    let additional_span = Span::new(span_base_acme.1, unsafe { span_base_acme.1.byte_add(size) });

    if let Some(address_space_lock_ptr) = ADDRESS_SPACE_TABLE.try_get_element_arc(KERNEL_ASID) {
        let mut address_space = address_space_lock_ptr.write();
        for i in (additional_span.get_base_acme().unwrap().0 as usize
            ..additional_span.get_base_acme().unwrap().1 as usize)
            .step_by(PAGE_SIZE)
        {
            if let Ok(frame) =
                crate::memory::allocator::PHYSICAL_FRAME_ALLOCATOR.lock().allocate_frame()
            {
                address_space
                    .map_page(crate::memory::vmem::MemoryMapping {
                        vaddr: VAddr::from(i),
                        paddr: frame,
                        page_type: crate::memory::vmem::PageType::KernelData,
                    })
                    .map_err(|_| ())?;
            } else {
                return Err(());
            }
        }
        Ok(additional_span)
    } else {
        Err(())
    }
}
