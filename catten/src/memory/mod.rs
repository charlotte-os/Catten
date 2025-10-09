pub mod allocator;
pub mod pmem;
pub mod vmem;

pub use pmem::{MemoryInterface, PAddr, PhysicalFrameAllocator};
pub use spin::{Lazy, Mutex, RwLock};
pub use vmem::VAddr;

pub use crate::common::collections::id_table::IdTable;
pub use crate::cpu::isa::interface::memory::AddressSpaceInterface;
pub use crate::cpu::isa::memory::paging::AddressSpace;
use crate::environment::boot_protocol::limine::{HHDM_REQUEST, MEMORY_MAP_REQUEST};

pub type AddressSpaceId = usize;

pub const KERNEL_ASID: AddressSpaceId = 0;
type AddressSpaceTable = IdTable<AddressSpaceId, AddressSpace>;
pub static ADDRESS_SPACE_TABLE: Lazy<AddressSpaceTable> = Lazy::new(|| AddressSpaceTable::new());

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
        MEMORY_MAP_REQUEST.get_response().expect("Limine failed to provide a memory map."),
    ))
});
