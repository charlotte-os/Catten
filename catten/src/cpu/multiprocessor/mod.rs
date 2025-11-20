//! # Multi-Processor Management
pub mod ipi;
pub mod lp_local;
pub mod startup;

#[inline]
pub fn get_lp_count() -> u32 {
    *(startup::LP_COUNT).read()
}
