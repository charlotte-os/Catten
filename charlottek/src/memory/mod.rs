pub mod allocator;
pub mod linear;
pub mod physical;
pub mod stack;

pub use linear::VAddr;
pub use physical::{MemoryIfce, PAddr, PhysicalFrameAllocator};
pub use spin::{Lazy, Mutex, RwLock};

pub use crate::common::collections::id_table::IdTable;
pub use crate::cpu::isa::interface::memory::AddressSpaceInterface;
pub use crate::cpu::isa::memory::paging::AddressSpace;
use crate::environment::boot_protocol::limine::{HHDM_REQUEST, MEMORY_MAP_REQUEST};

pub type AddressSpaceId = usize;
type AddressSpaceTable = IdTable<AddressSpaceId, AddressSpace>;

pub const KERNEL_ASID: AddressSpaceId = 0;

pub static ADDRESS_SPACE_TABLE: Lazy<AddressSpaceTable> = Lazy::new(|| AddressSpaceTable::new());
pub static HHDM_BASE: Lazy<VAddr> = Lazy::new(|| {
    VAddr::from(
        HHDM_REQUEST
            .get_response()
            .expect("Limine failed to provide a direct mapping region.")
            .offset() as usize,
    )
});
pub static PHYSICAL_FRAME_ALLOCATOR: Lazy<Mutex<PhysicalFrameAllocator>> = Lazy::new(|| {
    Mutex::new(PhysicalFrameAllocator::from(
        MEMORY_MAP_REQUEST.get_response().expect("Limine failed to provide a memory map."),
    ))
});
