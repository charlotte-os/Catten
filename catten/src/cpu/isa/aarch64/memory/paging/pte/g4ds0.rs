//! # 4KiB Granule, TCR_EL1.DS == 0
//! Ref: ARM ARM D.3.1

const VALID_DESC_BIT_INDEX: u64 = 0;
const TABLE_DESC_BIT_INDEX: u64 = 1;
const ACCESS_FLAG_BIT_INDEX: u64 = 10;
const NEXT_LEVEL_TABLE_ADDR_MASK: u64 = 0x0000_ffff_ffff_f000;

pub struct PageTableEntry(u64);
