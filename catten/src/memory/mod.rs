//! # Memory Management Subsystem

pub mod allocators;
pub mod linear;
pub mod physical;

pub use linear::VAddr;
pub use physical::{MemoryInterface, PAddr, PhysicalFrameAllocator};
pub use spin::{Lazy, Mutex, RwLock};

pub use crate::common::collections::id_table::IdTable;
pub use crate::cpu::isa::interface::memory::AddressSpaceInterface;
pub use crate::cpu::isa::memory::paging::AddressSpace;
use crate::environment::boot_protocol::limine::{HHDM_REQUEST, MEMORY_MAP_REQUEST};

pub type AddressSpaceId = usize;

/*The kernel address space is always ASID 0 and it is handled differently from userspace address
 * spaces because it needs to be initialized and accessible before the kernel allocator is
 * constructed and initialized.
 */
/// The kernel address space ID.
pub const KERNEL_ASID: AddressSpaceId = 0;
/// The kernel address space. It is initialized to the current address space when this static is
/// first accessed. Which should happen during the BSP init process.
pub static KERNEL_AS: Lazy<Mutex<AddressSpace>> =
    Lazy::new(|| Mutex::new(AddressSpace::get_current()));
/// Holds all userspace address spaces, indexed by their kernel assigned AddressSpaceId.
type AddressSpaceTable = IdTable<AddressSpaceId, AddressSpace>;
pub static ADDRESS_SPACE_TABLE: Lazy<AddressSpaceTable> = Lazy::new(|| AddressSpaceTable::new());
/// The starting virtual address of the higher half direct mapping region created by the bootloader.
/// This should be remapped by the VMM during BSP init to be placed at the address specified by the
/// kernel virtual memory map at which point this address should be updated to reflect the new
/// location.
pub static HHDM_BASE: Lazy<VAddr> = Lazy::new(|| {
    VAddr::from(
        HHDM_REQUEST
            .get_response()
            .expect("Limine failed to provide a higher half direct mapping region.")
            .offset() as usize,
    )
});
/// The physical frame allocator instance used by the kernel.
pub static PHYSICAL_FRAME_ALLOCATOR: Lazy<Mutex<PhysicalFrameAllocator>> = Lazy::new(|| {
    Mutex::new(PhysicalFrameAllocator::from(
        MEMORY_MAP_REQUEST.get_response().expect("Limine failed to provide a memory map."),
    ))
});
