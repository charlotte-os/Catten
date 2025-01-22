
struct PageTableWalker {
    p4: *mut PageTable,
    p3: *mut PageTable,
    p2: *mut PageTable,
    p1: *mut PageTable,
    p4_index: usize,
    p3_index: usize,
    p2_index: usize,
    p1_index: usize,
}