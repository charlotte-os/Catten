//! # Page Table Entry (PTE)

/* ARM paging is quite sophisticated with a number of different granule sizes and configurations, each
with differing page table entry formats. Each supported format is broken out into its own module. */
pub mod g4ds0; // 4KiB Granule, TCR_EL1.DS == 0
