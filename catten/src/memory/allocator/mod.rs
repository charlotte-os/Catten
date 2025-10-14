mod memory;

use core::mem::MaybeUninit;

use spin::{Lazy, Mutex};
use talc::*;

use crate::common::size::mebibytes;
use crate::cpu::isa::interface::memory::address::{Address, VirtualAddress};
use crate::cpu::isa::memory::paging::PAGE_SIZE;
use crate::memory::allocator::memory::try_allocate_and_map_range;
use crate::memory::linear::VAddr;
use crate::memory::linear::address_map::LA_MAP;
use crate::memory::linear::address_map::RegionType::KernelStackArena;

const INITIAL_HEAP_SIZE: usize = mebibytes(2);

static PRIMARY_ALLOCATOR: Lazy<Talck<Mutex<()>, ExtendOnOom>> =
    Lazy::new(|| Talck::new(Talc::new(ExtendOnOom::new())));
static HEAP_SPAN: Mutex<MaybeUninit<Span>> = Mutex::new(MaybeUninit::uninit());

struct ExtendOnOom {
    heap_span: Span,
}

impl ExtendOnOom {
    fn new() -> Self {
        let base = LA_MAP.get_region(KernelStackArena).base;
        try_allocate_and_map_range(base, INITIAL_HEAP_SIZE / PAGE_SIZE)
            .expect("Failed to allocate and map initial kernel heap memory");
        ExtendOnOom {
            heap_span: Span::new(base.into_mut(), (base + INITIAL_HEAP_SIZE).into_mut()),
        }
    }
}

impl OomHandler for ExtendOnOom {
    fn handle_oom(talc: &mut Talc<Self>, _layout: core::alloc::Layout) -> Result<(), ()> {
        let raw_span = unsafe { HEAP_SPAN.lock().assume_init() }.get_base_acme().unwrap();
        let (base, acme) = (VAddr::from_ptr(raw_span.0), VAddr::from_ptr(raw_span.1));
        let current_size = acme - base;
        let new_acme = core::cmp::min(
            acme + current_size,
            LA_MAP.get_region(KernelStackArena).base + LA_MAP.get_region(KernelStackArena).length,
        );
        let new_span = Span::new(base.into_mut(), new_acme.into_mut());
        if let Ok(_) = try_allocate_and_map_range(acme, current_size as usize / PAGE_SIZE) {
            HEAP_SPAN.lock().write(new_span);
            unsafe { talc.extend(Span::new(base.into_mut(), acme.into_mut()), new_span) };
            Ok(())
        } else {
            Err(())
        }
    }
}
