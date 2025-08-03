//! Physical Memory Access Control Structures

pub struct PhysicalMemoryAccessDescriptor {
    base: PAddr,
    frame_count: usize,
}

#[pack_bools]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalMemoryPermissions {
    map_writeable: bool,
    map_executable: bool,
    deallocate: bool,
}
