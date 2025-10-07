pub struct LinearAddressMap {
    null_page: LinearMemoryRegion,
    application: LinearMemoryRegion,
    direct_mapping: LinearMemoryRegion,
    kernel_stack_arena: LinearMemoryRegion,
    kernel_mmio: LinearMemoryRegion,
    kenrnel_allocator_arena: LinearMemoryRegion,
    kernel_image: LinearMemoryRegion,
}

pub struct LinearMemoryRegion {
    base: super::VAddr,
    length: usize,
}
