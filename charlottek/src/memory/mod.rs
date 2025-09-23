pub mod allocator;
pub mod pmem;
pub mod vmem;

pub use pmem::{MemoryInterface, PAddr, PhysicalFrameAllocator};
pub use spin::{Lazy, Mutex, RwLock};
pub use vmem::VAddr;

use crate::environment::boot_protocol::limine::{HHDM_REQUEST, MEMEORY_MAP_REQUEST};
pub use crate::isa::interface::memory::AddressSpaceInterface;
pub use crate::isa::memory::paging::AddressSpace;
pub use crate::klib::collections::id_table::IdTable;

pub type AddressSpaceId = usize;

pub const KERNEL_ASID: AddressSpaceId = 0;

pub static ADDRESS_SPACE_TABLE: Lazy<RwLock<AddressSpaceTable>> =
    Lazy::new(|| RwLock::new(AddressSpaceTable::new()));

type AddressSpaceTable = IdTable<AddressSpaceId, AddressSpace>;

pub static HHDM_BASE: Lazy<VAddr> = Lazy::new(|| {
    VAddr::from(
        HHDM_REQUEST
            .get_response()
            .expect("Limine failed to provide a higher half direct mapping region.")
            .offset() as usize,
    )
});
pub static PHYSICAL_FRAME_ALLOCATOR: Lazy<Mutex<PhysicalFrameAllocator>> = Lazy::new(|| {
    Mutex::new(PhysicalFrameAllocator::from(
        MEMEORY_MAP_REQUEST.get_response().expect("Limine failed to provide a memory map."),
    ))
});
